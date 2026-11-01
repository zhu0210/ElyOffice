use std::io::{self, PipeReader, PipeWriter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use ely_ipc::{Pack, read_frame, write_frame};

use super::Worker;
use crate::proto::{self, tag};

static ENDED: AtomicBool = AtomicBool::new(false);

/// Greets as this build, refuses the rest, notes its end.
fn fake(mut input: PipeReader, mut output: PipeWriter) -> io::Result<()> {
    while let Some((kind, id, _)) = read_frame(&mut input)? {
        match kind {
            tag::HELLO => write_frame(&mut output, tag::OK, id, &Pack::new().str(proto::BUILD).bytes())?,
            _ => write_frame(&mut output, tag::ERR, id, b"fake refuses")?,
        }
    }
    ENDED.store(true, Ordering::SeqCst);
    Ok(())
}

fn stranger(_: PipeReader, mut output: PipeWriter) -> io::Result<()> {
    write_frame(&mut output, tag::OK, 1, &Pack::new().str("another build").bytes())
}

#[test]
fn a_thread_worker_answers_frames_and_ends_with_its_client() {
    let mut worker = Worker::in_process(fake).expect("the thread greets");
    let refused = worker.request(tag::CLOSE, &[]).expect_err("the fake refuses");
    assert_eq!(refused.to_string(), "fake refuses");
    assert_eq!(worker.ended(), None);
    assert_eq!(worker.pid(), std::process::id());
    assert_eq!(worker.kill().expect_err("threads stay").kind(), io::ErrorKind::Unsupported);
    drop(worker);
    let start = Instant::now();
    while !ENDED.load(Ordering::SeqCst) {
        assert!(start.elapsed() < Duration::from_secs(5), "the thread outlived its client");
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn a_thread_worker_of_another_build_is_refused() {
    let Err(error) = Worker::in_process(stranger) else { panic!("a stranger is refused") };
    assert!(error.to_string().contains("from other sources"), "{error}");
}
