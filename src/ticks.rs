use std::sync::mpsc::{channel, Sender};
use std::thread::{spawn, sleep, JoinHandle};
use std::time::Duration;

enum Message {
  Tick,
  Stop,
}

#[derive(Debug)]
pub struct Ticks {
  sender: Sender<Message>,
}

impl Ticks {
  pub fn start() -> (Ticks, JoinHandle<()>) {
    println!("Starting Ticks...");
    let (sender, receiver) = channel::<Message>();

    let ticks = Ticks { sender };

    let handle = spawn(move || {

      for nessage in receiver {
        match nessage {
          Message::Tick => {
            Ticks::tick();
          },
          Message::Stop => {
            println!("Stopping Ticks...");
            break;
          },
        }
      }
    });

    let sender_ticks = ticks.sender.clone();

    spawn(move || {
      // tick every second
      let tick_interval = Duration::from_secs(1);

      loop {
        sender_ticks.send(Message::Tick).unwrap();
        sleep(tick_interval);
      }
    });

    (ticks, handle)
  }

  pub fn tick() {
    println!("Tick")
  }

  pub fn stop(&self) {
    self.sender.send(Message::Stop).unwrap();
  }
}
