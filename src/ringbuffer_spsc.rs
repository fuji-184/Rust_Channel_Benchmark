use ringbuffer_spsc::ringbuffer;
use std::thread;
use std::time::Instant;

use crate::NUM_MESSAGES;
use crate::BUFFER_SIZE;
 
pub fn spsc() {
    let (mut writer, mut reader) = ringbuffer::<usize>(BUFFER_SIZE);

    let start = Instant::now();

    let producer = thread::spawn(move || {
        for i in 0..NUM_MESSAGES {
            while writer.push(i).is_some() {
                thread::yield_now();
            }
        }
    });

    let consumer = thread::spawn(move || {
        let mut received = 0;
        while received < NUM_MESSAGES {
            match reader.pull() {
                Some(_) => received += 1,
                None => thread::yield_now(),
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let elapsed = start.elapsed().as_millis();
    println!("ringbuffer_spsc: {} ms", elapsed);
}
