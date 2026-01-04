#[allow(unused_imports)]
use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Plugin {
  pub name: String,
  pub times: Vec<f64>,
}

impl Plugin {
  /// Compute the average plugin start time.
  pub fn average(&self) -> f64 {
    let len = self.times.len();
    self.times.iter().sum::<f64>()
      / f64::from(u32::try_from(len).unwrap_or(u32::MAX))
  }

  /// Compute the standard deviation among all plugin start times.
  pub fn deviation(&self) -> f64 {
    let avg = self.average();
    let len = self.times.len();

    let variance = self
      .times
      .iter()
      .map(|value| {
        let diff = avg - (*value);
        diff * diff
      })
      .sum::<f64>()
      / f64::from(u32::try_from(len).unwrap_or(u32::MAX));

    variance.sqrt()
  }

  /// Compute the longest plugin start time
  pub fn max(&self) -> f64 {
    self.times.iter().copied().fold(f64::NAN, f64::max)
  }

  /// Compute the median plugin start time.
  pub fn median(&self) -> f64 {
    let mut values = self.times.clone();

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = values.len() / 2;
    if values.len().is_multiple_of(2) {
      f64::midpoint(values[mid - 1], values[mid])
    } else {
      values[mid]
    }
  }

  /// Compute the shortest plugin start time
  pub fn min(&self) -> f64 {
    self.times.iter().copied().fold(f64::NAN, f64::min)
  }

  pub fn new(name: String, times: Vec<f64>) -> Self {
    Self { name, times }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug)]
  pub struct Fixture {
    pub average: f64,
    pub deviation: f64,
    pub key: String,
    pub max: f64,
    pub median: f64,
    pub min: f64,
  }

  impl Fixture {
    pub fn new(
      key: String,
      average: f64,
      median: f64,
      deviation: f64,
      min: f64,
      max: f64,
    ) -> Self {
      Self {
        average,
        deviation,
        key,
        max,
        median,
        min,
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
      Fixture::new(
        String::from("vim-rooter"),
        6.72500,
        7.2,
        3.355_126_674_210_677,
        2.0,
        10.5,
      ),
      Fixture::new(
        String::from("vim-prettier"),
        8.0,
        4.65,
        6.499_615_373_235_557_5,
        3.5,
        19.2,
      ),
      Fixture::new(
        String::from("vim-just"),
        4.75,
        4.6,
        2.075_451_758_051_725_5,
        2.0,
        7.8,
      ),
    ];

    for (a, b) in fake {
      plugins.insert(
        a.clone(),
        Plugin {
          name: a.clone(),
          times: b.clone(),
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
    let (plugins, res) = setup();

    for fixture in res {
      assert!(approx_eq!(
        f64,
        plugins[&fixture.key].max(),
        fixture.max,
        ulps = 2
      ));
    }
  }

  #[test]
  fn min() {
    let (plugins, res) = setup();

    for fixture in res {
      assert!(approx_eq!(
        f64,
        plugins[&fixture.key].min(),
        fixture.min,
        ulps = 2
      ));
    }
  }
}
