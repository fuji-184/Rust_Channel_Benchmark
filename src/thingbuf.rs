use tokio::task;
use std::thread;
use std::time::Instant;
use thingbuf::mpsc;
use thingbuf::mpsc::blocking;

use crate::NUM_MESSAGES;
use crate::BUFFER_SIZE;

pub async fn bounded_async() {
    let (tx, rx) = mpsc::channel::<usize>(BUFFER_SIZE);
    let start = tokio::time::Instant::now();

    let producer = task::spawn(async move {
        for i in 0..NUM_MESSAGES {
            tx.send(i).await.unwrap();
        }
    });

    let consumer = task::spawn(async move {
        for _ in 0..NUM_MESSAGES {
            rx.recv().await.unwrap();
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();

    let elapsed = start.elapsed().as_millis();
    println!("thingbuf::bounded_async: {} ms", elapsed);
}

pub fn bounded_blocking() {
    let (tx, rx) = blocking::channel::<usize>(BUFFER_SIZE);
    let start = Instant::now();

    let producer = thread::spawn(move || {
        for i in 0..NUM_MESSAGES {
            tx.send(i).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        for _ in 0..NUM_MESSAGES {
            rx.recv().unwrap();
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let elapsed = start.elapsed().as_millis();
    println!("thingbuf::bounded_blocking: {} ms", elapsed);
}