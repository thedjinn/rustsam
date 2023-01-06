use rodio::{OutputStream, Sink};
use rodio::buffer::SamplesBuffer;

use rustsam::reciter;
use rustsam::parser;
use rustsam::renderer;

fn main() {
    let text = "test";

    let phrase = reciter::text_to_phonemes(text).expect("Could not recite text");
    let phonemes = parser::parse_phonemes(&phrase).expect("Could not parse phonemes");
    let output = renderer::render(&phonemes, 100, 127, 127, 80, false);

    //std::fs::write("/tmp/output.raw", &output).expect("Could not write output file");

    // Play audio file
    let (_, stream_handle) = OutputStream::try_default().expect("Could not open audio device");
    let sink = Sink::try_new(&stream_handle).expect("Could not create audio sink");
    sink.append(SamplesBuffer::new(1, 22050, output.into_iter().map(|sample| sample as u16 * 256).collect::<Vec<_>>()));
    sink.sleep_until_end();
}
