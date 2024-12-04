use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};



fn main() {
    let handles: Vec<_> = (1..=3)
        .map(|i|{
            thread::spawn(move|| {
                println!("Thread {i}");
                thread::sleep(Duration::from_millis(1));
            })
        })
        .collect();
    for handle in handles{
        handle.join().unwrap();
    }
    
    println!("All threads completed!");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5{
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move|| {
            for _ in 0..10{
                let mut num = counter_clone.lock().unwrap();
                *num += 1
            }
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());
}

