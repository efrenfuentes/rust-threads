use tokio::sync::mpsc::{channel, Sender};
use tokio::task::JoinHandle;
use tokio::spawn;

pub fn start_counter() -> (Sender<u64>, JoinHandle<()>) {
    println!("Starting Simple Counter...");
    let (tx, mut rx) = channel::<u64>(100);

    let handle = spawn(async move {
        let mut counter: u64 = 0;

        while let Some(value) = rx.recv().await {
            counter += value;
            println!("Counter updated: {}", counter);
        }

        println!("Stopping Simple Counter...");
    });

    (tx, handle)
}
