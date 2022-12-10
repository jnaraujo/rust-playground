// use std::io;

mod fib;

fn main() {

  let start_time = std::time::Instant::now();

  let n = fib::Fib::fib(100);

  let end_time = std::time::Instant::now();

  println!("fib(10) = {}", n);

  println!("Time elapsed: {} ms", end_time.duration_since(start_time).as_millis());

}