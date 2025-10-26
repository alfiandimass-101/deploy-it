use std::thread::{self, JoinHandle};

static mut COUNTER: u128 = 0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut thread_collection: Vec<JoinHandle<()>> = Vec::default();
    for _ in 0..10 {
        thread_collection.push(thread::spawn(|| {
            unsafe {
                COUNTER += 1;
            }
        }));
    }

    for handle in thread_collection {
        handle.join().unwrap();
    }

    let current_counter = unsafe { COUNTER };
    println!("{}", current_counter);
    Ok(())
}