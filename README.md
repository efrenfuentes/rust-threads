# Threads in Rust

Some examples of using threads in Rust.

## Running

To run the examples, use `cargo run <example_name>`. For example:

```bash
cargo run simple_counter
```

## Examples

### simple_counter

A thread with only one async method to keep a counter.

### counter

A thread with a counter that can be incremented and decremented.

### buffer

A thread with a buffer where you can push lines, when buffer is full, it will flush the lines.

### ticks

A thread that will print a message every second.
