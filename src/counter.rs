use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::{spawn, JoinHandle};

enum Message {
  Increment(u64),
  Decrement(u64),

  Value,
  ResultValue(u64),

  Stop,
}

#[derive(Debug)]
pub struct Counter {
  // input channel for sending messages to the counter
  sender: Sender<Message>,

  // output channel for receiving messages from the counter
  receiver: Receiver<Message>,
}

impl Counter {
  pub fn start() -> (Counter, JoinHandle<()>) {
    println!("Starting Counter...");
    let (input_sender, input_receiver) = channel::<Message>();
    let (output_sender, output_receiver) = channel::<Message>();

    let counter = Counter {
      sender: input_sender,
      receiver: output_receiver,
    };

    let handle = spawn(move || {
      let mut value: u64 = 0; // state: counter value

      for message in input_receiver {
        match message {
          Message::Increment(amount) => {
            value += amount;
            println!("Counter updated: {}", value)
          },
          Message::Decrement(amount) => {
            value -= amount;
            println!("Counter updated: {}", value)
          },
          Message::Value => {
            output_sender.send(Message::ResultValue(value)).unwrap()
          },
          Message::Stop => {
            println!("Stopping Counter...");
            break;
          },
          _ => {}
        }
      }
    });

    (counter, handle)
  }

  pub fn increment(&self, value: u64) {
    self.sender.send(Message::Increment(value)).unwrap();
  }

  pub fn decrement(&self, value: u64) {
    self.sender.send(Message::Decrement(value)).unwrap();
  }

  pub fn value(&self) -> u64 {
    self.sender.send(Message::Value).unwrap();

    match self.receiver.recv().unwrap() {
      Message::ResultValue(value) => value,
      _ => 0,
    }
  }

  pub fn stop(&self) {
    self.sender.send(Message::Stop).unwrap();
  }

}
