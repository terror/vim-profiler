use crate::common::*;

pub(crate) trait Plugins {
  fn sort(&mut self, reversed: bool) -> Vec<Plugin>;
  fn max(&self) -> f64;
  fn min(&self) -> f64;
  fn len_largest(&self) -> usize;
}

impl Plugins for Vec<Plugin> {
  fn sort(&mut self, reversed: bool) -> Vec<Plugin> {
    let sorted = self;

    sorted.sort_by(|a, b| {
      if reversed {
        a.average().partial_cmp(&b.average()).unwrap()
      } else {
        b.average().partial_cmp(&a.average()).unwrap()
      }
    });

    sorted.to_vec()
  }

  fn max(&self) -> f64 {
    self
      .iter()
      .max_by(|a, b| a.average().partial_cmp(&b.average()).unwrap())
      .unwrap()
      .average()
  }

  fn min(&self) -> f64 {
    self
      .iter()
      .min_by(|a, b| a.average().partial_cmp(&b.average()).unwrap())
      .unwrap()
      .average()
  }

  fn len_largest(&self) -> usize {
    self
      .iter()
      .min_by(|a, b| b.name.len().cmp(&a.name.len()))
      .unwrap()
      .name
      .len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn setup() -> Vec<Plugin> {
    [
      (String::from("vim-rooter"), vec![2.0, 5.2, 9.2, 10.5]),
      (String::from("vim-prettier"), vec![4.0, 5.3, 3.5, 19.2]),
      (String::from("vim-just"), vec![5.0, 2.0, 4.2, 7.8]),
    ]
    .iter()
    .map(|(a, b)| Plugin {
      name: a.to_owned(),
      times: b.to_owned(),
    })
    .collect()
  }

  #[test]
  fn sort() {
    let mut plugins = setup();

    let order = [
      String::from("vim-prettier"),
      String::from("vim-rooter"),
      String::from("vim-just"),
    ];

    for (i, plugin) in plugins.sort(false).iter().enumerate() {
      assert_eq!(plugin.name, order[i]);
    }
  }

  #[test]
  fn sort_reversed() {
    let mut plugins = setup();

    let order = [
      String::from("vim-just"),
      String::from("vim-rooter"),
      String::from("vim-prettier"),
    ];

    for (i, plugin) in plugins.sort(true).iter().enumerate() {
      assert_eq!(plugin.name, order[i]);
    }
  }

  #[test]
  fn max() {
    let plugins = setup();
    assert!(approx_eq!(f64, plugins.max(), 8.0, ulps = 2));
  }

  #[test]
  fn min() {
    let plugins = setup();
    assert!(approx_eq!(f64, plugins.min(), 4.75, ulps = 2));
  }

  #[test]
  fn len_largest() {
    let plugins = setup();
    assert_eq!(plugins.len_largest(), 12_usize);
  }
}
