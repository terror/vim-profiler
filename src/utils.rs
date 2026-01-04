use crate::common::*;

/// Repeats the string `s`, `n` times.
pub(crate) fn repeat(s: &str, n: usize) -> String {
  iter::repeat_n(s, n).collect::<Vec<_>>().join("")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_repeat() {
    let cases = vec![
      ("-", 5, "-----"),
      ("=", 10, "=========="),
      ("a", 12, "aaaaaaaaaaaa"),
    ];

    for (a, b, c) in cases {
      assert_eq!(repeat(a, b), c);
    }
  }
}
