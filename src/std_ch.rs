use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::NUM_MESSAGES;

pub fn unbounded() {
    let (tx, rx) = mpsc::channel::<usize>();

    let start = Instant::now();

    let producer = thread::spawn(move || {
        for i in 0..NUM_MESSAGES {
            tx.send(i).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        for _ in 0..NUM_MESSAGES {
            let _ = rx.recv().unwrap();
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let elapsed = start.elapsed().as_millis();
    println!("std::channel (unbounded): {} ms", elapsed);
}


