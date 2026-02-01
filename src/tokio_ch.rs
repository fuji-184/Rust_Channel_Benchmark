use tokio::sync::mpsc;
use tokio::time::Instant;
use crate::NUM_MESSAGES;

pub async fn unbounded() {
    let (tx, mut rx) = mpsc::unbounded_channel::<usize>();

    let start = Instant::now();

    let producer = tokio::spawn(async move {
        for i in 0..NUM_MESSAGES {
            tx.send(i).unwrap();
        }
    });

    let consumer = tokio::spawn(async move {
        for _ in 0..NUM_MESSAGES {
            let _ = rx.recv().await.unwrap();
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();

    let elapsed = start.elapsed().as_millis();
    println!("tokio::unbounded: {} ms", elapsed);
}