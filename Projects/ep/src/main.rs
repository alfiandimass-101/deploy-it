use std::thread::{self, JoinHandle};

static mut COUNTER: u128 = 0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:p}", &raw const COUNTER);
    let mut thread_collection: Vec<JoinHandle<()>> = Vec::default();
    for _ in 0..10 {
        thread_collection.push(thread::spawn(|| {
            for _ in 0..1_000_000 {
                unsafe {
                    COUNTER += 1;
                }
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