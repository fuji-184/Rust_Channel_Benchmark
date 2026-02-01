use std::thread;
use std::time::Instant;
use ringbuf::{traits::*, HeapRb};

use crate::NUM_MESSAGES;
//use crate::BUFFER_SIZE;
 
pub fn bounded() {
    let rb = HeapRb::<usize>::new(NUM_MESSAGES);
    let (mut tx, mut rx) = rb.split();

    let start = Instant::now();

    let producer = thread::spawn(move || {
        for i in 0..NUM_MESSAGES {
            tx.try_push(i).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        let mut received = 0;
        while received < NUM_MESSAGES {
            match rx.try_pop() {
                Some(_) => received += 1,
                None => thread::yield_now(),
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let elapsed = start.elapsed().as_millis();
    println!("ringbuf bounded: {} ms", elapsed);
}
