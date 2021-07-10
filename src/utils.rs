use crate::common::*;

/// Converts a map of String -> Vec<f64> to one of String -> f64
/// by taking the average of each value.
pub fn convert(data: &HashMap<String, Vec<f64>>) -> HashMap<String, f64> {
  data
    .iter()
    .map(|(k, v)| (k.to_owned(), v.iter().sum::<f64>() / v.len() as f64))
    .collect::<HashMap<String, f64>>()
}

/// A special purpose function to retrieve the length of the
/// largest word in a reference to a slice of `(&String, &f64)`.
pub fn len_largest(v: &[(String, f64)]) -> usize {
  v.iter()
    .min_by(|a, b| b.0.len().cmp(&a.0.len()))
    .unwrap()
    .0
    .len()
}

/// Repeats the string `s`, `n` times.
pub fn repeat(s: &str, n: usize) -> String {
  iter::repeat(s).take(n).collect::<Vec<_>>().join("")
}

/// Sorts a map of String -> f64.
/// Note that a times map sorted in `reverse`
/// indicates that the map is sorted in increasing order.
/// We assume the user would like to see longest times first (what's
/// slowing vim down).
pub fn sort(data: &HashMap<String, f64>, reversed: bool) -> Vec<(String, f64)> {
  let mut sorted = data.iter().collect::<Vec<(&String, &f64)>>();

  sorted.sort_by(|a, b| {
    if reversed {
      a.1.partial_cmp(b.1).unwrap()
    } else {
      b.1.partial_cmp(a.1).unwrap()
    }
  });

  sorted.iter().map(|(k, v)| (k.to_string(), **v)).collect()
}

#[cfg(test)]
mod tests {
  use crate::common::*;

  fn setup_map() -> HashMap<String, f64> {
    let mut data: HashMap<String, f64> = HashMap::new();

    let fake = vec![
      (String::from("vim-rooter"), 2.0),
      (String::from("vim-prettier"), 4.0),
      (String::from("vim-just"), 5.0),
      (String::from("vim-airline"), 6.0),
    ];

    for (a, b) in fake {
      data.insert(a, b);
    }

    data
  }

  #[test]
  fn len_largest() {
    let data = setup_map();

    let vec: Vec<(&String, &f64)> = data.iter().collect();

    assert_eq!(
      utils::len_largest(
        &vec
          .iter()
          .map(|(k, v)| (k.to_string(), **v))
          .collect::<Vec<(String, f64)>>()
      ),
      12
    );
  }

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

  #[test]
  fn sort() {
    let sorted = utils::sort(&setup_map(), false);

    let order = vec![
      (String::from("vim-airline"), 6.0),
      (String::from("vim-just"), 5.0),
      (String::from("vim-prettier"), 4.0),
      (String::from("vim-rooter"), 2.0),
    ];

    for (i, (k, v)) in sorted.iter().enumerate() {
      let (name, time) = &order[i];
      assert_eq!(*name, *k);
      assert!(approx_eq!(f64, *time, *v, ulps = 2));
    }
  }

  #[test]
  fn sort_reverse() {
    let sorted = utils::sort(&setup_map(), true);

    let order = vec![
      (String::from("vim-rooter"), 2.0),
      (String::from("vim-prettier"), 4.0),
      (String::from("vim-just"), 5.0),
      (String::from("vim-airline"), 6.0),
    ];

    for (i, (k, v)) in sorted.iter().enumerate() {
      let (name, time) = &order[i];
      assert_eq!(*name, *k);
      assert!(approx_eq!(f64, *time, *v, ulps = 2));
    }
  }

  #[test]
  fn convert() {
    let mut data: HashMap<String, Vec<f64>> = HashMap::new();

    let fake = vec![
      (String::from("vim-rooter"), vec![2.0, 5.2, 9.2, 10.5]),
      (String::from("vim-prettier"), vec![4.0, 5.3, 3.5, 19.2]),
      (String::from("vim-just"), vec![5.0, 2.0, 4.2, 7.8]),
      (String::from("vim-airline"), vec![6.0, 5.1, 5.2, 9.0]),
    ];

    for (a, b) in fake {
      data.insert(a, b);
    }

    let converted = utils::convert(&data);

    let res = vec![
      (String::from("vim-rooter"), 6.72500),
      (String::from("vim-prettier"), 8.0),
      (String::from("vim-just"), 4.75),
      (String::from("vim-airline"), 6.32500),
    ];

    for (name, avg) in res {
      assert!(approx_eq!(f64, converted[&name], avg, ulps = 2));
    }
  }
}
