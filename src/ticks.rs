use tokio::sync::mpsc::{channel, Sender};
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use std::time::Duration;

#[derive(Debug)]
enum Command {
    Tick,

    Stop,
}

pub struct Ticks {
    tx: Sender<Command>,
}

impl Ticks {
  pub fn start() -> (Ticks, JoinHandle<()>) {
    println!("Starting Ticks...");
    let (tx, mut rx) = channel::<Command>(100);

    let handle = spawn(async move {

      while let Some(command) = rx.recv().await {
        match command {
          Command::Tick => {
            Ticks::tick();
          },
          Command::Stop => {
            println!("Stopping Ticks...");
            break;
          },
        }
      }
    });

    let tx_ticks = tx.clone();

    spawn(async move {
      loop {
        tx_ticks.send(Command::Tick).await.unwrap();
        sleep(Duration::from_secs(1)).await;
      }
    });

    (Ticks { tx }, handle)
  }

  pub fn tick() {
    println!("Tick")
  }

  pub async fn stop(&mut self) {
    self.tx.send(Command::Stop).await.unwrap();
  }

}
