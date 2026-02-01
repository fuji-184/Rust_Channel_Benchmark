mod crossbeam;
mod kanal;
mod tachyonix;
mod flume;
mod crossfire;
mod tokio_ch;
mod std_ch;
mod ringbuffer_spsc;
mod thingbuf;
mod ringbuf;

use tokio::runtime::Runtime;

pub const NUM_MESSAGES: usize = 1_000_000_000;
pub const BUFFER_SIZE: usize = 1024;

fn main() {
    println!("\n\nMessages: {}\n\n\nsync channel :\n", NUM_MESSAGES);
    
    crossbeam::unbounded();
    kanal::unbounded();
    crossfire::unbounded();
    std_ch::unbounded();
    ringbuffer_spsc::spsc();
    thingbuf::bounded_blocking();
    ringbuf::bounded();
    
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        println!("\n\nasync channel :\n");
        tachyonix::unbounded().await;
        kanal::unbounded_async().await;
        flume::unbounded().await;
        crossfire::unbounded_async().await;
        tokio_ch::unbounded().await;
        thingbuf::bounded_async().await;
        println!("\n");
    });
}
