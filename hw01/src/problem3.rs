
fn is_prime(n: u32) -> bool{
  for i in 2 .. n {
    if n % i == 0 {
      return false;
    }
  }

  return true;
}

/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
  let mut prime: Vec<u32> = Vec::new();
  if n == 0 {
    return prime;
  }

  for i in 2 .. n {
    if is_prime(i) {
      prime.push(i);
    }
  }
  return prime;
}