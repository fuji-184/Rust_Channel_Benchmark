use tachyonix::channel;
use crate::NUM_MESSAGES;
use tokio::time::Instant;

pub async fn unbounded() {
    
        let (tx, mut rx) = channel::<usize>(NUM_MESSAGES); // kapasitas besar agar "unbounded"

        let start = Instant::now();

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
        println!("tachyonix::unbounded: {} ms", time);
    
}
