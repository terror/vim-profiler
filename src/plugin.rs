#[allow(unused_imports)]
use crate::common::*;

#[derive(Debug, Clone)]
pub struct Plugin {
  pub name:  String,
  pub times: Vec<f64>,
}

impl Plugin {
  pub fn new(name: String, times: Vec<f64>) -> Self {
    Self { name, times }
  }

  /// Compute the average plugin start time.
  pub fn average(&self) -> f64 {
    self.times.iter().sum::<f64>() / self.times.len() as f64
  }

  /// Compute the standard deviation among all plugin start times.
  pub fn deviation(&self) -> f64 {
    let avg = self.average();

    let variance = self
      .times
      .iter()
      .map(|value| {
        let diff = avg - (*value);
        diff * diff
      })
      .sum::<f64>()
      / self.times.len() as f64;

    variance.sqrt()
  }

  /// Compute the median plugin start time.
  pub fn median(&self) -> f64 {
    let mut values = self.times.clone();

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
      (values[mid - 1] + values[mid]) / 2.0
    } else {
      values[mid]
    }
  }

  /// Compute the longest plugin start time
  pub fn max(&self) -> f64 {
    self.times.iter().cloned().fold(f64::NAN, f64::max)
  }

  /// Compute the shortest plugin start time
  pub fn min(&self) -> f64 {
    self.times.iter().cloned().fold(f64::NAN, f64::min)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug)]
  pub struct Fixture {
    pub key:       String,
    pub average:   f64,
    pub median:    f64,
    pub deviation: f64,
  }

  impl Fixture {
    pub fn new(key: String, average: f64, median: f64, deviation: f64) -> Self {
      Self {
        key,
        average,
        median,
        deviation,
      }
    }
  }

  fn setup() -> (HashMap<String, Plugin>, Vec<Fixture>) {
    let mut plugins: HashMap<String, Plugin> = HashMap::new();

    let fake = vec![
      (String::from("vim-rooter"), vec![2.0, 5.2, 9.2, 10.5]),
      (String::from("vim-prettier"), vec![4.0, 5.3, 3.5, 19.2]),
      (String::from("vim-just"), vec![5.0, 2.0, 4.2, 7.8]),
    ];

    let res = vec![
      Fixture::new(String::from("vim-rooter"), 6.72500, 7.2, 3.355126674210677),
      Fixture::new(String::from("vim-prettier"), 8.0, 4.65, 6.4996153732355575),
      Fixture::new(String::from("vim-just"), 4.75, 4.6, 2.0754517580517255),
    ];

    for (a, b) in fake {
      plugins.insert(
        a.to_owned(),
        Plugin {
          name:  a.to_owned(),
          times: b.to_owned(),
        },
      );
    }

    (plugins, res)
  }

  #[test]
  fn average() {
    let (plugins, res) = setup();

    for fixture in res {
      assert!(approx_eq!(
        f64,
        plugins[&fixture.key].average(),
        fixture.average,
        ulps = 2
      ));
    }
  }

  #[test]
  fn median() {
    let (plugins, res) = setup();

    for fixture in res {
      assert!(approx_eq!(
        f64,
        plugins[&fixture.key].median(),
        fixture.median,
        ulps = 2
      ));
    }
  }

  #[test]
  fn deviation() {
    let (plugins, res) = setup();

    for fixture in res {
      assert!(approx_eq!(
        f64,
        plugins[&fixture.key].deviation(),
        fixture.deviation,
        ulps = 2
      ));
    }
  }

  #[test]
  fn max() {
    let (plugins, _) = setup();
    assert!(approx_eq!(
      f64,
      plugins
        .iter()
        .map(|(_, v)| v.to_owned())
        .collect::<Vec<Plugin>>()
        .max(),
      8.0,
      ulps = 2
    ));
  }

  #[test]
  fn min() {
    let (plugins, _) = setup();
    assert!(approx_eq!(
      f64,
      plugins
        .iter()
        .map(|(_, v)| v.to_owned())
        .collect::<Vec<Plugin>>()
        .min(),
      4.75,
      ulps = 2
    ));
  }
}
