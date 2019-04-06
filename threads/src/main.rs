use std::thread;
use std::time;

fn main() {
    let mut thread_handles = vec![];
    for i in 0..5 {
        thread_handles.push(thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(20));
            println!("hello from thread {}!", i);
        }));
    }

    for i in 0..5 {
        println!("hello {} from main!", i);
        thread::sleep(time::Duration::from_millis(20));
    }

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
}
