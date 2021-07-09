use crate::common::*;

pub struct Stats {
  data: HashMap<String, f64>,
}

impl Stats {
  pub fn new(data: HashMap<String, f64>) -> Self {
    Self { data }
  }

  /// Compute the average plugin start time.
  pub fn average(&self) -> f64 {
    let values = self.data.iter().map(|(_, v)| *v).collect::<Vec<f64>>();
    values.iter().sum::<f64>() / values.len() as f64
  }

  /// Compute the standard deviation among all plugin start times.
  pub fn deviation(&self) {}

  /// Compute the median plugin start time.
  pub fn median(&self) {}

  /// Retrieve the plugin with the longest start time.
  // pub fn longest(&self) -> (String, f64) {
  //   self
  //     .data
  //     .iter()
  //     .max_by(|a, b| a.1.cmp(&b.1))
  //     .map(|(k, v)| (k.to_string(), *v))
  // }

  /// Retrieve the plugin with the shortest start time.
  // pub fn shortest(&self) -> (String, f64) {
  //   self
  //     .data
  //     .iter()
  //     .max_by(|a, b| a.1.cmp(&b.1))
  //     .map(|(k, v)| (k.to_string(), *v))
  // }

  /// Plot the statistics to the terminal.
  pub fn plot(&self) -> Result<()> {
    Ok(())
  }

  /// Write the statistics to a CSV file.
  pub fn write(&self) -> Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn average() {}

  #[test]
  fn deviation() {}

  #[test]
  fn median() {}

  #[test]
  fn longtest() {}

  #[test]
  fn shortest() {}
}
