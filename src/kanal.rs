use std::{thread, time::Instant};
use crate::NUM_MESSAGES;
use kanal;
use kanal::{AsyncSender, AsyncReceiver};

pub fn unbounded() {
    let (tx, rx) = kanal::unbounded::<usize>(); // unbounded sync channel

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
    println!("kanal::unbounded: {} ms", time);
}

pub async fn unbounded_async() {
    
        let (tx, rx): (AsyncSender<usize>, AsyncReceiver<usize>) = kanal::unbounded_async();

        let start = tokio::time::Instant::now();

        let producer = tokio::spawn(async move {
            for i in 0..NUM_MESSAGES {
                tx.send(i).await.unwrap();
            }
        });

        let consumer = tokio::spawn(async move {
            for _ in 0..NUM_MESSAGES {
                rx.recv().await.unwrap();
            }
        });

        producer.await.unwrap();
        consumer.await.unwrap();

        let time = start.elapsed().as_millis();
        println!("kanal::unbounded_async: {} ms", time);
  
}
