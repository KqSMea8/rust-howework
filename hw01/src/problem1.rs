use std::collections::HashMap;

pub fn sum(slice: &[i32]) -> i32 {
  let mut total: i32 = 0;
  for num in slice {
    total = total + num;
  }
  return total;
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
  let mut remove_dup_map = HashMap::new();
  let mut distinct_vec: Vec<i32> = Vec::new();

  for elem in vs {
    if !remove_dup_map.contains_key(&elem) {
      distinct_vec.push(*elem);
      remove_dup_map.insert(elem, true);
    }
  }
  return distinct_vec;
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut satisfy_vec: Vec<i32> = Vec::new();
    for elem in vs {
      if pred(*elem) {
        satisfy_vec.push(*elem);
      }
    }
    return satisfy_vec;
}
