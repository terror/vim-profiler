use crate::common::*;

pub struct Stats {
  data: HashMap<String, f64>,
}

impl Stats {
  pub fn new(data: HashMap<String, f64>) -> Self {
    Self { data }
  }

  /// Grab the values out of `data`.
  pub fn values(&self) -> Vec<f64> {
    self.data.iter().map(|(_, v)| *v).collect::<Vec<f64>>()
  }

  /// Compute the average plugin start time.
  pub fn average(&self) -> f64 {
    let values = &self.values();
    values.iter().sum::<f64>() / values.len() as f64
  }

  /// Compute the standard deviation among all plugin start times.
  pub fn deviation(&self) -> f64 {
    let avg = self.average();

    let variance = self
      .values()
      .iter()
      .map(|value| {
        let diff = avg - (*value);
        diff * diff
      })
      .sum::<f64>()
      / self.values().len() as f64;

    variance.sqrt()
  }

  /// Compute the median plugin start time.
  pub fn median(&self) -> f64 {
    let values = &mut self.values();

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
      (values[mid - 1] + values[mid]) / 2.0
    } else {
      values[mid]
    }
  }

  /// Retrieve the plugin with the longest start time.
  pub fn longest(&self) -> (String, f64) {
    self
      .data
      .iter()
      .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
      .map(|(k, v)| (k.to_string(), *v))
      .unwrap()
  }

  /// Retrieve the plugin with the shortest start time.
  pub fn shortest(&self) -> (String, f64) {
    self
      .data
      .iter()
      .max_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
      .map(|(k, v)| (k.to_string(), *v))
      .unwrap()
  }

  /// Plot the statistics to the terminal.
  pub fn plot(&self) -> Result<()> {
    let width = 1200;
    let height = 800;
    let (top, right, bottom, left) = (90, 10, 50, 120);

    let x = ScaleLinear::new()
      .set_domain(vec![0_f32, (self.longest().1 + 1.0) as f32])
      .set_range(vec![0, width - left - right]);

    let y = ScaleBand::new()
      .set_domain(
        sort_times(&self.data, false)
          .iter()
          .map(|(k, _)| k.to_owned())
          .collect(),
      )
      .set_range(vec![0, height - top - bottom]);

    let view = HorizontalBarView::new()
      .set_x_scale(&x)
      .set_y_scale(&y)
      .load_data(
        &self
          .data
          .iter()
          .map(|(k, v)| (k.to_owned(), *v as f32))
          .collect::<Vec<(String, f32)>>(),
      )
      .unwrap();

    Chart::new()
      .set_width(width)
      .set_height(height)
      .set_margins(top, right, bottom, left)
      .add_title(String::from("Vim Plugin Start Times"))
      .add_view(&view)
      .add_axis_bottom(&x)
      .add_axis_top(&x)
      .add_axis_left(&y)
      .add_left_axis_label("Name")
      .add_bottom_axis_label("Time")
      .save("plugins.svg")
      .unwrap();

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

  fn setup() -> Stats {
    let mut data: HashMap<String, f64> = HashMap::new();

    let fake = vec![
      (String::from("vim-rooter"), 2.0),
      (String::from("vim-prettier"), 3.0),
      (String::from("vim-just"), 5.0),
      (String::from("vim-airline"), 6.0),
    ];

    for (a, b) in fake {
      data.insert(a, b);
    }

    Stats::new(data)
  }

  #[test]
  fn average() {
    let stats = setup();
    assert!(approx_eq!(f64, stats.average(), 4.0, ulps = 2));
  }

  #[test]
  fn deviation() {
    let stats = setup();
    assert!(approx_eq!(
      f64,
      stats.deviation(),
      1.5811388300841898,
      ulps = 2
    ),);
  }

  #[test]
  fn median() {
    let stats = setup();
    assert!(approx_eq!(f64, stats.median(), 4.0, ulps = 2));
  }

  #[test]
  fn longest() {
    let stats = setup();
    assert_eq!(stats.longest(), (String::from("vim-airline"), 6.0));
  }

  #[test]
  fn shortest() {
    let stats = setup();
    assert_eq!(stats.shortest(), (String::from("vim-rooter"), 2.0));
  }
}
