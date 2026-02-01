use crossfire::spsc;
use tokio::time::Instant;
use tokio::task;
use std::thread;
use crate::NUM_MESSAGES;

// Sync unbounded
pub fn unbounded() {
    let (tx, rx) = spsc::unbounded_blocking::<usize>();

    let start = Instant::now();

    let producer = thread::spawn(move || {
        for i in 0..NUM_MESSAGES {
            tx.send(i).expect("send ok");
        }
    });

    let consumer = thread::spawn(move || {
        for _ in 0..NUM_MESSAGES {
            rx.recv().expect("recv ok");
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let time = start.elapsed().as_millis();
    println!("crossfire::unbounded: {} ms", time);
}

// Async unbounded
pub async fn unbounded_async() {
    let (tx, rx) = spsc::unbounded_async::<usize>();

    let start = Instant::now();

    let producer = task::spawn(async move {
        for i in 0..NUM_MESSAGES {
            tx.send(i).expect("send ok");
        }
    });

    let consumer = task::spawn(async move {
        for _ in 0..NUM_MESSAGES {
            rx.recv().await.expect("recv ok");
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();

    let time = start.elapsed().as_millis();
    println!("crossfire::unbounded_async: {} ms", time);
}
