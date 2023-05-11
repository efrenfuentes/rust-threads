use tokio::sync::mpsc::{channel, Sender, Receiver};
use tokio::spawn;
use tokio::task::JoinHandle;

#[derive(Debug)]
enum Command {
    Increment {
      value: u64,
    },
    Decrement {
      value: u64,
    },
    Value,
    Stop,
}

#[derive(Debug)]
enum Output {
    Value {
      value: u64,
    },
}

pub struct Counter {
    tx: Sender<Command>,
    rx: Receiver<Output>,
}

impl Counter {
  pub fn start() -> (Counter, JoinHandle<()>) {
    println!("Starting Counter...");
    let (input_tx, mut input_rx) = channel::<Command>(100);
    let (output_tx, output_rx) = channel::<Output>(100);

    let handle = spawn(async move {
      let mut counter: u64 = 0;

      while let Some(command) = input_rx.recv().await {
        match command {
          Command::Increment {value} => {
            counter += value;
            println!("Counter updated: {}", counter);
          },
          Command::Decrement {value} => {
            counter -= value;
            println!("Counter updated: {}", counter);
          },
          Command::Value => {
            output_tx.send(Output::Value {value: counter}).await.unwrap();
          },
          Command::Stop => {
            println!("Stopping Counter...");
            break;
          },
        }
      }
    });

    (Counter { tx: input_tx, rx: output_rx }, handle)
  }

  pub async fn increment(&mut self, value: u64) {
    self.tx.send(Command::Increment {value}).await.unwrap();
  }

  pub async fn decrement(&mut self, value: u64) {
    self.tx.send(Command::Decrement {value}).await.unwrap();
  }

  pub async fn value(&mut self) -> u64 {
    self.tx.send(Command::Value).await.unwrap();

    match self.rx.recv().await.unwrap() {
      Output::Value {value} => value,
    }
  }

  pub async fn stop(&mut self) {
    self.tx.send(Command::Stop).await.unwrap();
  }

}
