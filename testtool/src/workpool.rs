use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::thread::JoinHandle;

use parking_lot::Mutex;

struct Worker {
    process: Child,
    stdout_handle: Option<JoinHandle<()>>,
    stderr_handle: Option<JoinHandle<()>>,
    port: usize
}

pub struct WorkPool(Mutex<Vec<Option<Worker>>>);

impl WorkPool {
    pub fn new() -> Self {
        Self(Mutex::new(Vec::new()))
    }

    pub fn allocate_worker(&self) -> usize {
        let thread_index = rayon::current_thread_index().expect("Not running in a thread");

        // Check if there is an existing worker that can be reused
        if let Some(worker) = self.0.lock().get(thread_index).and_then(|worker| worker.as_ref()) {
            return worker.port;
        }

        // Start a new worker
        let port = 6100 + thread_index;

        let mut process = Command::new("node")
            .arg("maketest.mjs")
            .arg("--")
            .arg(port.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Could not execute maketests.mjs");

        // TODO: Add timeout
        let mut stdout = BufReader::new(process.stdout.take().unwrap());

        let mut line = String::new();
        stdout.read_line(&mut line).expect("Could not read stdout from nodejs worker");

        if line.trim() != "ready" {
            panic!("Received unexpected stdout response from nodejs worker");
        }

        // Start draining stdout and stderr
        let stdout_handle = std::thread::spawn(|| {
            for line in stdout.lines() {
                let line = line.expect("Could not read from stdin");
                println!("node stdin: {}", line);
            }
        });

        let stderr = process.stderr.take().unwrap();
        let stderr_handle = std::thread::spawn(|| {
            for line in BufReader::new(stderr).lines() {
                let line = line.expect("Could not read from stdin");
                println!("node stderr: {}", line);
            }
        });

        // Populate work pool
        let mut workers = self.0.lock();

        if workers.len() < thread_index + 1 {
            workers.resize_with(thread_index + 1, || None);
        }

        workers[thread_index] = Some(Worker {
            process,
            stdout_handle: Some(stdout_handle),
            stderr_handle: Some(stderr_handle),
            port
        });

        port
    }
}

impl Drop for WorkPool {
    fn drop(&mut self) {
        println!("Shutting down nodejs workers");

        let mut workers = self.0.lock();

        for worker in workers.iter_mut() {
            if let Some(worker) = worker.as_mut() {
                // TODO: Shut down gracefully by sending a message
                match worker.process.kill() {
                    Err(err) if err.kind() == std::io::ErrorKind::InvalidInput => {
                        println!("Warning: could not kill nodejs worker because it was not running, crashed?");
                    },
                    Err(err) => {
                        println!("Warning: could not kill nodejs worker: {:?}", err);
                    },
                    _ => ()
                }

                worker.process.wait().expect("Wait on process failed");

                worker.stdout_handle.take().unwrap().join().expect("Could not join stdout collection thread");
                worker.stderr_handle.take().unwrap().join().expect("Could not join stderr collection thread");
            }

            *worker = None;
        }

        workers.clear();
    }
}
