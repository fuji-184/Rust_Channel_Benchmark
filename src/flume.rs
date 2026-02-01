use flume::Sender;
use flume::Receiver;
use tokio::time::Instant;
use tokio::task;
use crate::NUM_MESSAGES;

pub async fn unbounded() {
    let (tx, rx): (Sender<usize>, Receiver<usize>) = flume::unbounded();

    let start = Instant::now();

    let producer = task::spawn({
        let tx = tx.clone();
        async move {
            for i in 0..NUM_MESSAGES {
                tx.send_async(i).await.unwrap();
            }
        }
    });

    let consumer = task::spawn(async move {
        for _ in 0..NUM_MESSAGES {
            rx.recv_async().await.unwrap();
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();

    let time = start.elapsed().as_millis();
    println!("flume::unbounded: {} ms", time);
}
