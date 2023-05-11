use std::env::args;
use std::time::Duration;
use tokio::time::sleep;

mod simple_counter;
mod counter;
mod buffer;
mod ticks;

#[tokio::main]
async fn main() {
    match args().nth(1) {
        Some(arg) => match arg.as_ref() {
            "simple_counter" => simple_counter().await,
            "counter" => counter().await,
            "buffer" => buffer().await,
            "ticks" => ticks().await,
            _ => println!("Unknown argument: {}", arg),
        },
        None => println!("No argument provided"),
    }
}

async fn simple_counter() {
    let (tx, handle) = simple_counter::start_counter();

    tx.send(1).await.unwrap();
    tx.send(5).await.unwrap();
    tx.send(1).await.unwrap();

    sleep(Duration::from_secs(1)).await;

    drop(tx);

    handle.await.unwrap();
}

async fn counter() {
    let (mut counter, handle) = counter::Counter::start();

    counter.increment(1).await;
    counter.increment(5).await;
    counter.increment(10).await;
    counter.decrement(3).await;

    let current_value = counter.value().await;
    println!("Current value: {}", current_value);

    sleep(Duration::from_secs(1)).await;

    counter.stop().await;

    handle.await.unwrap();
}

async fn buffer() {
    let (mut buffer, handle) = buffer::Buffer::start();

    for i in 1..26 {
        buffer.add_line(format!("Line {}", i)).await;
    }

    sleep(Duration::from_secs(1)).await;

    buffer.stop().await;

    handle.await.unwrap();
}

async fn ticks() {
    let (mut ticks, handle) = ticks::Ticks::start();

    sleep(Duration::from_secs(5)).await;

    ticks.stop().await;

    handle.await.unwrap();
}
