use std::sync::mpsc::{channel, Sender};
use std::thread::{spawn, JoinHandle};

enum Message {
  AddLine(String),

  Stop,
}

#[derive(Debug)]
pub struct Buffer {
  // input channel for sending messages to the counter
  sender: Sender<Message>,
}

impl Buffer {
  pub fn start() -> (Buffer, JoinHandle<()>) {
    println!("Starting Buffer...");
    let (sender, receiver) = channel::<Message>();

    let buffer = Buffer { sender };

    let handle = spawn(move || {
      let mut state: Vec<String> = Vec::new();

      for message in receiver {
        match message {
          Message::AddLine(line) => {
            state.push(line);

            if state.len() >= 10 {
              Buffer::flush(&mut state)
            }
          },
          Message::Stop => {
            println!("Stopping Buffer...");
            Buffer::flush(&mut state);
            break;
          },
        }
      }
    });

    (buffer, handle)
  }

  fn flush(state: &mut Vec<String>) {
    println!("Flushing buffer: {:?}", state);
    state.clear();
  }

  pub fn add_line(&self, value: String) {
    self.sender.send(Message::AddLine(value)).unwrap();
  }

  pub fn stop(&self) {
    self.sender.send(Message::Stop).unwrap();
  }

}
