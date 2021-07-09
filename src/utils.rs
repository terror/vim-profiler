use crate::common::*;

/// A special purpose function to retrieve the length of the
/// largest word in a reference to a slice of `(&String, &f64)`.
pub fn len_largest(v: Vec<(String, f64)>) -> usize {
  v.iter()
    .min_by(|a, b| b.0.len().cmp(&a.0.len()))
    .unwrap()
    .0
    .len()
}

/// Repeats the string `s`, `n` times.
pub fn str_repeat(s: &str, n: usize) -> String {
  iter::repeat(s).take(n).collect::<Vec<_>>().join("")
}

/// Sorts a map of String -> f64
pub fn sort_times(data: &HashMap<String, f64>, reversed: bool) -> Vec<(String, f64)> {
  let mut sorted: Vec<(&String, &f64)> = data.iter().collect();

  if reversed {
    sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
  } else {
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
  }

  sorted.iter().map(|(k, v)| (k.to_string(), **v)).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_len_largest() {
    let mut data: HashMap<String, f64> = HashMap::new();

    data.insert(String::from("cool"), 2.0);
    data.insert(String::from("awesome"), 3.0);
    data.insert(String::from("yeet"), 4.0);

    let vec: Vec<(&String, &f64)> = data.iter().collect();

    assert_eq!(
      len_largest(vec.iter().map(|(k, v)| (k.to_string(), **v)).collect()),
      7
    );
  }

  #[test]
  fn test_str_repeat() {
    let cases = vec![
      ("-", 5, "-----"),
      ("=", 10, "=========="),
      ("a", 12, "aaaaaaaaaaaa"),
    ];

    for (a, b, c) in cases {
      assert_eq!(str_repeat(a, b), c);
    }
  }

  #[test]
  fn test_sort_times() {}
}
