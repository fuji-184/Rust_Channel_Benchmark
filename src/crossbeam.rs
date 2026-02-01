use std::{thread, time::Instant};
use crate::NUM_MESSAGES;
use crossbeam::channel as cb;

pub fn unbounded() {
    let (tx, rx) = cb::unbounded::<usize>();

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
    let time = start.elapsed().as_millis();
    println!("crossbeam::unbounded: {} ms", time);
}