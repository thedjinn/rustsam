use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use clap::{Args, Parser, Subcommand};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use parking_lot::Mutex;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

mod reference_json;
mod serialization;
mod utils;
mod workpool;

use serialization::{serialize_to_compressed_bincode, deserialize_from_compressed_bincode, deserialize_from_compressed_bincode_slice};
use workpool::WorkPool;
use utils::MinPrioritized;

#[derive(Debug)]
pub enum Error {
    Aborted,
    Error(&'static str),
    HTTPError(&'static str, Box<ureq::Error>), // note: huge struct
    IOError(&'static str, Box<std::io::Error>),
    JSONError(&'static str, Box<serde_json::Error>),
    BincodeError(&'static str, bincode::Error) // already boxed
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Aborted => write!(f, "Aborted"),
            Error::Error(message) => write!(f, "{}", message),
            Error::HTTPError(message, err) => write!(f, "{} ({})", message, err),
            Error::IOError(message, err) => write!(f, "{} ({})", message, err),
            Error::JSONError(message, err) => write!(f, "{} ({})", message, err),
            Error::BincodeError(message, err) => write!(f, "{} ({})", message, err)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Input {
    text: String,
    phonetic: bool,
    sing_mode: bool,
    pitch: u8,
    speed: u8,
    mouth: u8,
    throat: u8
}

#[derive(Debug, Deserialize, Serialize)]
struct Output {
    input: Input,
    recited: String,
    parsed: Vec<(u8, u8, u8)>,

    #[serde(with="serde_bytes")]
    rendered: Vec<u8>
}

impl Output {
    fn perform<F, T, E>(&self, expected: &T, closure: F) -> Outcome<T>
    where
        E: std::error::Error + Send + 'static,
        F: Fn() -> std::result::Result<T, E> + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        T: Clone + Eq + Send
    {
        let start = std::time::Instant::now();
        let result = std::panic::catch_unwind(closure);
        let duration = start.elapsed();

        match result {
            Ok(result) => {
                match result {
                    Ok(output) if output == *expected => Outcome::Success(duration),
                    Ok(output) => Outcome::WrongOutput {
                        duration,
                        output,
                        expected: expected.clone()
                    },
                    Err(error) => Outcome::Error {
                        duration,
                        error: Box::new(error)
                    }
                }
            },
            Err(error) => Outcome::Panic {
                duration,
                error
            }
        }
    }

    fn recite(&self) -> Outcome<String> {
        self.perform(&self.recited, || rustsam::reciter::text_to_phonemes(&self.input.text))
    }

    fn parse(&self) -> Outcome<Vec<rustsam::parser::Phoneme>> {
        // Map sam-js' (phoneme index, duration, stress) triple to Phoneme instances
        let expected = self.parsed.iter().map(|&(index, length, stress)| {
            rustsam::parser::Phoneme {
                index: index as usize,
                length,
                stress
            }
        }).collect::<Vec<_>>();

        self.perform(&expected, || rustsam::parser::parse_phonemes(&self.recited))
    }

    fn render(&self) -> Outcome<Vec<u8>> {
        // Map sam-js' (phoneme index, duration, stress) triple to Phoneme instances
        let phonemes = self.parsed.iter().map(|&(index, length, stress)| {
            rustsam::parser::Phoneme {
                index: index as usize,
                length,
                stress
            }
        }).collect::<Vec<_>>();

        self.perform(&self.rendered, || Ok::<_, Error>(rustsam::renderer::render(&phonemes, 64, 128, 128, 72, false)))
    }
}

#[derive(Debug)]
enum Outcome<T: Send> {
    Success(Duration),
    WrongOutput {
        duration: Duration,
        output: T,
        expected: T
    },
    Error {
        duration: Duration,
        error: Box<dyn std::error::Error + Send>
    },
    Panic {
        duration: Duration,
        error: Box<dyn std::any::Any + Send>
    }
}

impl<T: Send> Outcome<T> {
    fn passed(&self) -> bool {
        matches!(self, Outcome::Success(_))
    }

    fn panicked(&self) -> bool {
        matches!(self, Outcome::Panic { .. })
    }
}

impl<T: Send> std::fmt::Display for Outcome<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&match self {
            Outcome::Success(duration) => {
                format!("\x1b[1;92mPASS\x1b[0;32m in \x1b[92m{:?}\x1b[0m", duration)
            },
            Outcome::WrongOutput { duration, .. } => {
                format!("\x1b[1;91mFAIL\x1b[0;31m in \x1b[91m{:?}\x1b[0m", duration)
            },
            Outcome::Error { duration, .. } => {
                format!("\x1b[1;91mERROR\x1b[0;31m in \x1b[91m{:?}\x1b[0m", duration)
            },
            Outcome::Panic { duration, .. } => {
                format!("\x1b[1;91mPANIC\x1b[0;31m in \x1b[91m{:?}\x1b[0m", duration)
            }
        })
    }
}

struct DetailedOutcome<'a, T: Send>(&'a Outcome<T>);

impl<'a, T: std::fmt::Debug + Send> std::fmt::Display for DetailedOutcome<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Outcome::Success(_) => {
                write!(f, "")
            },
            Outcome::WrongOutput { output, expected, .. } => {
                write!(f, "Output:   {:?}\nExpected: {:?}", output, expected)
            },
            Outcome::Error { error, .. } => {
                write!(f, "Error:    {:?}", error)
            },
            Outcome::Panic { error, .. } => {
                write!(f, "Panic:    {:?}", error)
            }
        }
    }
}

const INPUTS: &[&str] = &[
    "SUCCESSFUL ",
    "'PRENTICE ",
    "ANISOGAMOUS ",
    "Hello, my name is SAM.",
    "is correct, play again? or do you prefer pong?",
    "just testing",
    "10.5%",
    "1 2 3 4 5 6 0.5, I CAN COUNT!",
    "computer",
    "HELLO",
    "WIZARD",
    "TWO THINGS",
    "ANOTHER TRAVELER",
    "HELLO, MY NAME IS SAM.",
    "THE SKY ABOVE THE PORT WAS THE COLOR OF TELEVISION, TUNED TO A DEADCHANNEL.",
    "ITS LIKE MY BODYS DEVELOPED THIS MASSIVE DRUG DEFICIENCY.",
    "IT WAS A SPRAWL VOICE AND A SPRAWL JOKE.",
    "RATZ WAS TENDING BAR, HIS PROSTHETIC ARM JERKING MONOTONOUSLY AS HE FILLEDA TRAY OF GLASSES WITH DRAFT KIRIN.",
    "HE SAW CASE AND SMILED, HIS TEETH A WEB WORK OF EAST EUROPEAN STEEL ANDBROWN DECAY.",
    "WAGE WAS IN HERE EARLY, WITH TWO JOE BOYS, RATZ SAID, SHOVING A DRAFTACROSS THE BAR WITH HIS GOOD HAND.",
    "MAYBE SOME BUSINESS WITH YOU, CASE. CASE SHRUGGED.",
    "THE GIRL TO HIS RIGHT GIGGLED AND NUDGED HIM.",
    "Sam is a very small Text-To-Speech (TTS) program written in C, that runs on most popular platforms.",
    "IT'S NOT LIKE I'M USING, CASE HEARD SOMEONE SAY, AS HE SHOULDERED HISWAY THROUGH THE CROWD AROUND THE DOOR OF THE CHAT.",
    "THE CHATSUBO WAS A BAR FOR PROFESSIONAL EXPATRIATES YOU COULD DRINK THEREFOR A WEEK AND NEVER HEAR TWO WORDS IN JAPANESE.",
    "CASE FOUND A PLACE AT THE BAR, BETWEEN THE UNLIKELY TAN ON ONE OF LONNYZONE'S WHORES AND THE CRISP NAVAL UNIFORM OF A TALL AFRICAN WHOSECHEEKBONES WERE RIDGED WITH PRECISE ROWS OF TRIBAL SCARS.",
    "Hello world",
    "Hello, world!",
    "Hello? World!"
];

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    name: String,
    offset: usize,
    size: usize
}

#[derive(Debug, Deserialize, Serialize)]
struct TestIndex {
    testcases: Vec<TestCase>
}

struct TestSet {
    index: TestIndex,
    reader: BufReader<File>
}

impl TestSet {
    fn from_files<P, Q>(index: P, data: Q) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>
    {
        let data_file = File::open(data).map_err(|err|
            Error::IOError("Could not open testset data file", Box::new(err))
        )?;

        let index = std::fs::read(index).map_err(|err|
            Error::IOError("Could not read testset index file", Box::new(err))
        )?;

        Ok(Self {
            index: deserialize_from_compressed_bincode_slice(&index)?,
            reader: BufReader::new(data_file)
        })
    }

    fn len(&self) -> usize {
        self.index.testcases.len()
    }

    fn load(&mut self, index: usize) -> Result<Vec<u8>, Error> {
        let Some(testcase) = self.index.testcases.get(index) else {
            return Err(Error::Error("Test case index is out of bounds"));
        };

        self.reader.seek(std::io::SeekFrom::Start(testcase.offset as u64)).map_err(|err|
            Error::IOError("Could not seek to testcase offset", Box::new(err))
        )?;

        let mut result = vec![0; testcase.size];
        self.reader.read_exact(&mut result).map_err(|err|
            Error::IOError("Could not read testcase contents", Box::new(err))
        )?;

        Ok(result)
    }

    fn iter(&mut self) -> TestSetIterator {
        TestSetIterator {
            buffer: Vec::with_capacity(32768),
            index_iter: self.index.testcases.iter(),
            reader: &mut self.reader
        }
    }
}

struct TestSetIterator<'a> {
    buffer: Vec<u8>,
    index_iter: std::slice::Iter<'a, TestCase>,
    reader: &'a mut BufReader<File>
}

impl Iterator for TestSetIterator<'_> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index_iter.next().map(|testcase| {
            if self.buffer.len() < testcase.size {
                self.buffer.resize(testcase.size, 0);
            }

            self.reader.read_exact(&mut self.buffer[..testcase.size]).expect("Read error");

            self.buffer[..testcase.size].to_owned()
        })
    }
}

struct TestDataWriter {
    index: TestIndex,
    data: BufWriter<File>,
    compressed_size: usize,
    uncompressed_size: usize
}

impl TestDataWriter {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::create(path).map_err(|err|
            Error::IOError("Could not open testdata file", Box::new(err))
        )?;

        Ok(Self {
            index: TestIndex {
                testcases: Vec::new()
            },
            data: BufWriter::new(file),
            compressed_size: 0,
            uncompressed_size: 0
        })
    }

    fn append(&mut self, name: String, data: &[u8], uncompressed_size: usize) -> Result<(), Error> {
        self.uncompressed_size += uncompressed_size;
        self.compressed_size += data.len();

        // Append to index
        let offset = self.data.stream_position().map_err(|err|
            Error::IOError("Could not get stream position of data file", Box::new(err))
        )? as usize;

        self.index.testcases.push(TestCase {
            name,
            offset,
            size: data.len()
        });

        // Append to data file
        self.data.write_all(data).map_err(|err|
            Error::IOError("Could not write to data file", Box::new(err))
        )?;

        Ok(())
    }
}

struct CompressedTestCase {
    compressed_data: Vec<u8>,
    uncompressed_size: usize,
    text: String
}

struct Generator {
    work_pool: WorkPool,
    writer: Mutex<TestDataWriter>,
    agent: Agent
}

impl Generator {
    fn new(writer: TestDataWriter) -> Self {
        let agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(30))
            .timeout_write(Duration::from_secs(10))
            .build();

        Self {
            agent,
            work_pool: WorkPool::new(),
            writer: Mutex::new(writer)
        }
    }

    fn into_inner(self) -> TestDataWriter {
        self.writer.into_inner()
    }

    fn generate_testcase(&self, text: String) -> Result<CompressedTestCase, Error> {
        let input = Input {
            text: text.clone(),
            phonetic: false,
            sing_mode: false,
            pitch: 64,
            speed: 72,
            mouth: 128,
            throat: 128
        };

        let input = serde_json::to_string(&input).map_err(|err|
            Error::JSONError("Could not serialize input", Box::new(err))
        )?;

        let port = self.work_pool.allocate_worker();

        let output: Output = self.agent.post(&format!("http://127.0.0.1:{}", port))
            .send_string(&input)
            .map_err(|err|
                Error::HTTPError("Could not issue HTTP request to nodejs worker", Box::new(err))
            )?
            .into_json()
            .map_err(|err|
                Error::IOError("Could not deserialize HTTP response from nodejs worker", Box::new(err))
            )?;

        let uncompressed_size = bincode::serialized_size(&output).map_err(|err|
            Error::BincodeError("Could not compute serialized size", err)
        )? as usize;

        let compressed_data = serialize_to_compressed_bincode(Vec::new(), &output)?;

        Ok(CompressedTestCase {
            compressed_data,
            uncompressed_size,
            text
        })
    }

    fn generate_testcases<T>(&self, inputs: &[T], aborted: Arc<AtomicBool>) -> Result<(), Error>
    where
        T: AsRef<str> + Sync
    {
        let last_index = inputs.len() - 1;

        let progress_bar = ProgressBar::new(inputs.len() as u64).with_style(make_progress_style());

        std::thread::scope(|scope| {
            let (tx, rx) = std::sync::mpsc::sync_channel(rayon::current_num_threads() * 2);

            let handle = scope.spawn(|| {
                // Let the thread take ownership of rx
                let rx = rx;

                // Lock the writer for the duration of the thread
                let mut writer = self.writer.lock();

                // Store incoming results in a heap so they can be ordered by test index
                let mut heap = BinaryHeap::<MinPrioritized<usize, Box<CompressedTestCase>>>::new();
                let mut next_index = 0;

                // Fetch results while the channel is open
                'outer: while let Ok((index, output)) = rx.recv() {
                    heap.push((index, output).into());

                    // Keep processing results as long as the test indices in the heap are increasing
                    // by one for every iteration, starting at the expected next index.
                    while let Some(output) = heap.peek() {
                        // Check if the sequence was broken
                        if output.priority != next_index {
                            break;
                        }

                        // Remove the result from the heap and increase the expected next index
                        // counter.
                        let output = heap.pop().unwrap();
                        next_index += 1;

                        // Append to the test data writer
                        let testcase = output.value;
                        writer.append(testcase.text, &testcase.compressed_data, testcase.uncompressed_size)?;

                        // Stop receiving results from the channel if the last index in the test set
                        // was pulled from the heap.
                        if output.priority == last_index {
                            break 'outer;
                        }
                    }
                }

                Ok(())
            });

            let result = inputs.par_iter().enumerate().progress_with(progress_bar).map(|(index, text)| {
                if aborted.load(Ordering::SeqCst) {
                    return Err(Error::Aborted);
                }

                // Check if the collection thread exited prematurely
                if handle.is_finished() {
                    // This will trigger the parallel iterator to abort, but the actual error is a
                    // stub that will be replaced by the error that is returned by handle.join().
                    return Err(Error::Aborted);
                }

                let output = self.generate_testcase(text.as_ref().to_owned())?;

                tx.send((index, Box::new(output))).map_err(|_| {
                    Error::Error("Could not send result to collection channel")
                })
            }).collect::<Result<(), Error>>();

            drop(tx);
            handle.join().map_err(std::panic::resume_unwind).unwrap()?;

            result
        })
    }
}

fn make_progress_style() -> ProgressStyle {
    ProgressStyle::with_template("[{eta_precise}] [{bar:80.green/white}] {pos}/{len} {msg}")
        .expect("Could not parse progress bar style")
        .progress_chars("## ")
}

fn generate_dataset(aborted: Arc<AtomicBool>) -> Result<(), Error> {
    // Collect the input entries
    let inputs = if true {
        reference_json::extract_inputs("./reference/reciter")?
    } else {
        INPUTS.iter().map(|&input| input.into()).collect()
    };

    println!("Collected {:?} input entries", inputs.len());

    // Generate testcases from the input entries
    let generator = Generator::new(TestDataWriter::new("/tmp/testdata").expect("Could not create test data writer"));
    generator.generate_testcases(&inputs, aborted).expect("Could not generate testcases");

    // Unwrap the test data writer index (and discard the data file handle)
    let TestDataWriter {
        index,
        compressed_size,
        uncompressed_size,
        ..
    } = generator.into_inner();

    // Serialize and write index
    serialize_to_compressed_bincode(
        BufWriter::new(
            File::create("/tmp/testindex").expect("Could not open testindex file")
        ),
        &index
    ).expect("Could not serialize index");

    // Print some statistics
    println!("Uncompressed size: {:8}", uncompressed_size);
    println!("Compressed size:   {:8}", compressed_size);
    println!("Compression ratio: {:7.2}% of original", (compressed_size as f32 / uncompressed_size as f32) * 100.0);

    // Perform a simple open/load verification on the generated files
    let mut testset = TestSet::from_files("/tmp/testindex", "/tmp/testdata").expect("Could not open testset for verification");
    assert_eq!(testset.len(), inputs.len());

    let testcase = testset.load(0).expect("Could not load first testset entry for validation");
    let testcase: Output = deserialize_from_compressed_bincode(&testcase[..]).expect("Could not deserialize first testset entry");
    assert_eq!(testcase.input.text, inputs[0]);

    Ok(())
}

struct TestResult {
    index: usize,
    text: String,
    reciter_result: Outcome<String>,
    parser_result: Outcome<Vec<rustsam::parser::Phoneme>>,
    renderer_result: Outcome<Vec<u8>>
}

impl TestResult {
    fn passed(&self) -> bool {
        self.reciter_result.passed() &&
        self.parser_result.passed() &&
        self.renderer_result.passed()
    }

    fn panicked(&self) -> bool {
        self.reciter_result.panicked() ||
        self.parser_result.panicked() ||
        self.renderer_result.panicked()
    }
}

impl std::fmt::Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_line_length = 30;

        write!(f, "\x1b[90m{:6} \x1b[94m{:max_line_length$}\x1b[90m -> \x1b[0m{:40} {:40} {:40}", self.index, self.text, self.reciter_result, self.parser_result, self.renderer_result)

        //match &self.reciter_result {
            //Outcome::Success(duration) => {
                //write!(f, "reciter: \x1b[1;92mPASS\x1b[0;32m in \x1b[92m{:10?}\x1b[0m", duration)
            //},
            //Outcome::WrongOutput { duration, output, expected } => {
                //write!(f, "reciter: \x1b[1;91mFAIL\x1b[0;31m (\x1b[91m{:?}\x1b[31m instead of \x1b[91m{:?}\x1b[31m) in \x1b[91m{:?}\x1b[0m", output, expected, duration)
            //},
            //Outcome::Error { duration, error } => {
                //write!(f, "reciter: \x1b[1;91mFAIL\x1b[0;31m ({:?}) in \x1b[91m{:?}\x1b[0m", error, duration)
            //},
            //Outcome::Panic { duration, error } => {
                //write!(f, "reciter: \x1b[1;91mFAIL\x1b[0;31m (Panic: {:?}) in \x1b[91m{:?}\x1b[0m", error, duration)
            //}
        //}?;

        //match &self.parser_result {
            //Outcome::Success(duration) => {
                //write!(f, "parser: \x1b[1;92mPASS\x1b[0;32m in \x1b[92m{:?}\x1b[0m", duration)
            //},
            //Outcome::WrongOutput { duration, output, expected } => {
                //write!(f, "parser: \x1b[1;91mFAIL\x1b[0;31m (\x1b[91m{:?}\x1b[31m instead of \x1b[91m{:?}\x1b[31m) in \x1b[91m{:?}\x1b[0m", output, expected, duration)
            //},
            //Outcome::Error { duration, error } => {
                //write!(f, "parser: \x1b[1;91mFAIL\x1b[0;31m ({:?}) in \x1b[91m{:?}\x1b[0m", error, duration)
            //},
            //Outcome::Panic { duration, error } => {
                //write!(f, "parser: \x1b[1;91mFAIL\x1b[0;31m (Panic: {:?}) in \x1b[91m{:?}\x1b[0m", error, duration)
            //}
        //}?;

        //match &self.renderer_result {
            //Outcome::Success(duration) => {
                //write!(f, "parser: \x1b[1;92mPASS\x1b[0;32m in \x1b[92m{:?}\x1b[0m", duration)
            //},
            //Outcome::WrongOutput { duration, output, expected } => {
                //write!(f, "parser: \x1b[1;91mFAIL\x1b[0;31m (\x1b[91m{:?}\x1b[31m instead of \x1b[91m{:?}\x1b[31m) in \x1b[91m{:?}\x1b[0m", output, expected, duration)
            //},
            //Outcome::Error { duration, error } => {
                //write!(f, "parser: \x1b[1;91mFAIL\x1b[0;31m ({:?}) in \x1b[91m{:?}\x1b[0m", error, duration)
            //},
            //Outcome::Panic { duration, error } => {
                //write!(f, "parser: \x1b[1;91mFAIL\x1b[0;31m (Panic: {:?}) in \x1b[91m{:?}\x1b[0m", error, duration)
            //}
        //}
    }
}

impl Eq for TestResult {}

impl PartialEq for TestResult {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl Ord for TestResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Order by ascending index
        other.index.cmp(&self.index)
    }
}

impl PartialOrd for TestResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct FailureReport<'a>(&'a TestResult);

impl std::fmt::Display for FailureReport<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = &self.0;

        if result.passed() {
            return Ok(());
        }

        if !result.reciter_result.passed() {
            write!(f, "{}", DetailedOutcome(&result.reciter_result))?;
        }

        if !result.parser_result.passed() {
            write!(f, "{}", DetailedOutcome(&result.parser_result))?;
        }

        if !result.renderer_result.passed() {
            write!(f, "{}", DetailedOutcome(&result.renderer_result))?;
        }

        Ok(())
    }
}

trait ConsoleWriter {
    fn println(&self, line: &str);
}

struct StdoutWriter;

impl ConsoleWriter for StdoutWriter {
    fn println(&self, line: &str) {
        println!("{}", line);
    }
}

impl ConsoleWriter for ProgressBar {
    fn println(&self, line: &str) {
        self.println(line);
    }
}

struct Context {
    aborted: Arc<AtomicBool>,
    show_diffs: bool,
    abort_on_failure: bool,
    abort_on_panic: bool,
    failures_only: bool,
    console_writer: Box<dyn ConsoleWriter + Sync>
}

impl Context {
    fn with_console_writer(mut self, console_writer: Box<dyn ConsoleWriter + Sync>) -> Self {
        self.console_writer = console_writer;
        self
    }
}

impl Context {
    fn println<T: AsRef<str>>(&self, line: T) {
        self.console_writer.println(line.as_ref());
    }
}

fn perform_test(_context: &Context, index: usize, testcase: &[u8]) -> Result<TestResult, Error> {
    let testcase: Output = deserialize_from_compressed_bincode_slice(testcase)?;

    Ok(TestResult {
        index,
        text: testcase.input.text.clone(),
        reciter_result: testcase.recite(),
        parser_result: testcase.parse(),
        renderer_result: testcase.render()
    })
}

fn analyze_test(context: &Context, result: &TestResult) {
    // Return if not interested in successes
    if context.failures_only && result.passed() {
        return;
    }

    // Pretty print the results
    context.println(result.to_string());

    if context.show_diffs && !result.passed() {
        context.println(FailureReport(result).to_string());
    }
}

fn run_test(context: &Context, index: usize) -> Result<(), Error> {
    // Load the dataset
    let mut testset = TestSet::from_files("/tmp/testindex", "/tmp/testdata")?;

    let testcase = testset.load(index)?;

    let result = perform_test(context, index, &testcase)?;
    analyze_test(context, &result);

    Ok(())
}

fn run_tests(context: Context) -> Result<(), Error> {
    // Load the dataset
    let mut testset = TestSet::from_files("/tmp/testindex", "/tmp/testdata")?;
    let last_index = testset.len() - 1;

    let progress_bar = ProgressBar::new(testset.len() as u64).with_style(make_progress_style());
    let b2 = progress_bar.clone();

    let context = context.with_console_writer(Box::new(progress_bar));

    std::thread::scope(|scope| {
        let (tx, rx) = std::sync::mpsc::sync_channel(rayon::current_num_threads() * 2);

        let handle = scope.spawn(|| {
            // Let the thread take ownership of rx
            let rx = rx;

            // Store incoming results in a heap so they can be ordered by test index
            let mut heap = BinaryHeap::<TestResult>::new();
            let mut next_index = 0;

            // Fetch results while the channel is open
            'outer: while let Ok(result) = rx.recv() {
                heap.push(result);

                // Keep processing results as long as the test indices in the heap are increasing
                // by one for every iteration, starting at the expected next index.
                while let Some(result) = heap.peek() {
                    // Check if the sequence was broken
                    if result.index != next_index {
                        break;
                    }

                    // Remove the result from the heap and increase the expected next index
                    // counter.
                    let result = heap.pop().unwrap();
                    next_index += 1;

                    analyze_test(&context, &result);

                    // Abort on first failure or panic if requested
                    if (context.abort_on_failure && !result.passed()) || (context.abort_on_panic && result.panicked()) {
                        break 'outer;
                    }

                    // Stop receiving results from the channel if the last index in the test set
                    // was pulled from the heap.
                    if result.index == last_index {
                        break 'outer;
                    }
                }
            }

            //println!("Finished collecting results");
        });

        let result = testset.iter().enumerate().par_bridge().progress_with(b2).map(|(index, testcase)| {
            if context.aborted.load(Ordering::SeqCst) {
                return Err(Error::Aborted);
            }

            let result = perform_test(&context, index, &testcase)?;

            tx.send(result).map_err(|_| {
                Error::Error("Could not send result to collection channel")
            })
        }).collect::<Result<(), Error>>();

        drop(tx);
        handle.join().expect("Could not join result collection thread");

        result
    })
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Run the test suite
    Run(Run),

    /// Generate test dataset
    Generate {
    }
}

#[derive(Args)]
struct Run {
    /// The test number to run (optional)
    test: Option<usize>,

    /// Exit on first failed test
    #[arg(short='x', long)]
    exit_on_failure: bool,

    /// Exit on first panicking test
    #[arg(short='p', long)]
    exit_on_panic: bool,

    /// Show differences for failed tests
    #[arg(short='d', long)]
    diff: bool,

    /// Show only failed tests
    #[arg(short='f', long)]
    failures_only: bool
}

impl Run {
    fn execute(&self, aborted: Arc<AtomicBool>) -> Result<(), Error> {
        let context = Context {
            aborted,
            show_diffs: self.diff,
            abort_on_failure: self.exit_on_failure,
            abort_on_panic: self.exit_on_panic,
            failures_only: self.failures_only,
            console_writer: Box::new(StdoutWriter)
        };

        if let Some(index) = self.test {
            run_test(&context, index)
        } else {
            run_tests(context)
        }
    }
}

fn main() -> Result<(), Error> {
    // Set a SIGINT handler
    let aborted = Arc::new(AtomicBool::new(false));

    let aborted_ctrlc = aborted.clone();
    ctrlc::set_handler(move || aborted_ctrlc.store(true, Ordering::SeqCst))
        .expect("Could not set interrupt signal handler");

    // Handle command line arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(run) => run.execute(aborted),
        Commands::Generate { } => generate_dataset(aborted)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
