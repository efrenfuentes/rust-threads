use std::sync::mpsc::{channel, Sender};
use std::thread::{spawn, JoinHandle};

pub fn start_counter() -> (Sender<u64>, JoinHandle<()>) {
    println!("Starting Simple Counter...");
    let (tx, rx) = channel::<u64>();

    let handle = spawn(move || {
        let mut counter: u64 = 0; // state

        for msg in rx { // receive messages to change the state
            counter += msg;

            println!("counter updated: {}", counter);
        }

        println!("Stopping Simple Counter...");
    });

    (tx, handle)
}
