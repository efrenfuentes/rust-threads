use tokio::sync::mpsc::{channel, Sender};
use tokio::spawn;
use tokio::task::JoinHandle;

#[derive(Debug)]
enum Command {
    AddLine {
      line: String,
    },

    Stop,
}

pub struct Buffer {
    tx: Sender<Command>,
}

impl Buffer {
  pub fn start() -> (Buffer, JoinHandle<()>) {
    println!("Starting Buffer...");
    let (tx, mut rx) = channel::<Command>(100);

    let handle = spawn(async move {
      let mut state: Vec<String> = Vec::new();

      while let Some(command) = rx.recv().await {
        match command {
          Command::AddLine {line} => {
            state.push(line);

            if state.len() >= 10 {
              Buffer::flush(&mut state).await;
            }
          },
          Command::Stop => {
            println!("Stopping Buffer...");
            Buffer::flush(&mut state).await;
            break;
          },
        }
      }
    });

    (Buffer { tx }, handle)
  }

  pub async fn add_line(&mut self, line: String) {
    self.tx.send(Command::AddLine {line}).await.unwrap();
  }

  pub async fn flush(state: &mut Vec<String>) {
    println!("Flushing buffer: {:?}", state);
    state.clear();
  }

  pub async fn stop(&mut self) {
    self.tx.send(Command::Stop).await.unwrap();
  }

}
