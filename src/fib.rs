use std::collections::HashMap;

pub struct Fib {}

impl Fib {
  pub fn fib(n: u32) -> u128 {
    let mut cache: HashMap<u32, u128> = HashMap::new();
    Fib::recursive_fib(n, &mut cache)
  }

  fn recursive_fib(n: u32, cache: &mut HashMap<u32, u128>) -> u128 {
    if n <= 2 {
      return 1;
    }

    if cache.contains_key(&n) {
      return cache[&n];
    }

    let result = Fib::recursive_fib(n - 1, cache) + Fib::recursive_fib(n - 2, cache);

    cache.insert(n, result);

    return result;
  }
}