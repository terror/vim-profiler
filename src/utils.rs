use crate::common::*;

/// Repeats the string `s`, `n` times.
pub fn repeat(s: &str, n: usize) -> String {
  iter::repeat(s).take(n).collect::<Vec<_>>().join("")
}

#[cfg(test)]
mod tests {
  use crate::common::*;

  #[test]
  fn test_repeat() {
    let cases = vec![
      ("-", 5, "-----"),
      ("=", 10, "=========="),
      ("a", 12, "aaaaaaaaaaaa"),
    ];

    for (a, b, c) in cases {
      assert_eq!(utils::repeat(a, b), c);
    }
  }
}
