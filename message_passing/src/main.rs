use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Sending messages and receiving with timeout:");
    let (tx, rx) = mpsc::channel();
    produce_nums(tx);
    receive_messages_with_timeout(rx);

    println!("\nSending messages and receiving using blocking iterator:");
    let (tx, rx) = mpsc::channel();
    produce_nums(tx);
    receive_messages(rx);
}

fn receive_messages(rx: Receiver<u32>) {
    for message in rx {
        println!("Got {}", message);
    }
}

fn receive_messages_with_timeout(rx: Receiver<u32>) {
    loop {
        match rx.recv_timeout(Duration::from_millis(1000)) {
            Ok(x) => println!("Got {}", x),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}

fn produce_nums(sender: Sender<u32>) {
    let sender1 = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        for i in 0..8 {
            thread::sleep(Duration::from_millis(3u64.pow(i)));
            sender1.send(i);
        }
    });

    let sender2 = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        for i in 8..12 {
            thread::sleep(Duration::from_millis(2u64.pow(i)));
            sender2.send(i);
        }
    });
}
