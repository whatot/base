use std::sync::mpsc;
use std::thread;

fn main() {}

#[test]
fn test_single_async_channel() {
    let num = 566;
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    thread::spawn(move || {
        tx.send(num).unwrap();
    });

    assert_eq!(num, rx.recv().unwrap());
}
