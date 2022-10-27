use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::mpsc;
use std::thread;

#[test]
fn test_channel_single_sender() {
    let num = 566;
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    thread::spawn(move || {
        tx.send(num).unwrap();
    });

    assert_eq!(num, rx.recv().unwrap());
}

#[test]
fn test_channel_multi_sender() {
    let thread_num = 32;
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    for id in 0..thread_num {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id).unwrap();
        });
    }

    let result_set: HashSet<i32> = HashSet::from_iter(0..thread_num);
    for _ in &result_set {
        assert!(result_set.contains(&rx.recv().unwrap()));
    }
}
