use std::env::args;
use std::thread::sleep;
use std::time::Duration;

mod simple_counter;
mod counter;
mod buffer;
mod ticks;

fn main() {
    match args().nth(1) {
        Some(arg) => match arg.as_ref() {
            "simple_counter" => simple_counter(),
            "counter" => counter(),
            "buffer" => buffer(),
            "ticks" => ticks(),
            _ => println!("Unknown argument: {}", arg),
        },
        None => println!("Please provide an argument: simple_counter, counter or buffer"),
    }
}

fn simple_counter() {
  let (sender, handle) = simple_counter::start_counter();

  sender.send(1).unwrap();
  sender.send(5).unwrap();
  sender.send(1).unwrap();

  sleep(Duration::from_millis(100));

  drop(sender);

  handle.join().unwrap();
}

fn counter() {
  let (counter, handle) = counter::Counter::start();

  counter.increment(1);
  counter.increment(5);
  counter.increment(10);
  counter.decrement(3);

  let current_value = counter.value();
  println!("Current value: {}", current_value);

  sleep(Duration::from_millis(100));

  counter.stop();

  handle.join().unwrap();
}

fn buffer() {
  let (buffer, handle) = buffer::Buffer::start();

  for i in 1..26 {
      let line = format!("Line {}", i);
      buffer.add_line(line);
  }

  sleep(Duration::from_millis(100));

  buffer.stop();

  handle.join().unwrap();
}

fn ticks() {
  let (ticks, handle) = ticks::Ticks::start();

  sleep(Duration::from_secs(5));

  ticks.stop();

  handle.join().unwrap();
}
