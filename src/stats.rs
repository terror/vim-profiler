use crate::common::*;

pub struct Stats {
  data: HashMap<String, Vec<f64>>,
}

impl Stats {
  pub fn new(data: HashMap<String, Vec<f64>>) -> Self {
    Self { data }
  }

  /// Compute the average plugin start time.
  pub fn average(&self, key: &str) -> f64 {
    self.data[key].iter().sum::<f64>() / self.data[key].len() as f64
  }

  /// Compute the standard deviation among all plugin start times.
  pub fn deviation(&self, key: &str) -> f64 {
    let avg = self.average(key);

    let variance = self.data[key]
      .iter()
      .map(|value| {
        let diff = avg - (*value);
        diff * diff
      })
      .sum::<f64>()
      / self.data[key].len() as f64;

    variance.sqrt()
  }

  /// Compute the median plugin start time.
  pub fn median(&self, key: &str) -> f64 {
    let mut values = self.data[key].clone();

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
    utils::convert(&self.data)
      .iter()
      .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
      .map(|(k, v)| (k.to_string(), *v))
      .unwrap()
  }

  /// Retrieve the plugin with the shortest start time.
  pub fn shortest(&self) -> (String, f64) {
    utils::convert(&self.data)
      .iter()
      .max_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
      .map(|(k, v)| (k.to_string(), *v))
      .unwrap()
  }

  /// Write the statistics to a CSV file.
  pub fn write(&self) -> Result<()> {
    let mut writer = Writer::from_path("plugins.csv").unwrap();

    writer
      .write_record(&["Plugin", "Average", "Median", "Deviation"])
      .unwrap();

    let data = utils::sort(&utils::convert(&self.data), false);

    for (k, _) in data.iter() {
      writer
        .write_record(&[
          k.to_owned(),
          format!("{:.5}", self.average(&k)),
          format!("{:.5}", self.median(&k)),
          format!("{:.5}", self.deviation(&k)),
        ])
        .unwrap();
    }

    writer.flush().unwrap();

    info!("Statistics written to `plugins.csv`");

    Ok(())
  }

  /// Plot the statistics and save it to a `.svg` file.
  pub fn plot(&self) -> Result<()> {
    let width = 1200;
    let height = 800;
    let (top, right, bottom, left) = (90, 10, 50, 120);
    let data = utils::convert(&self.data);

    let x = ScaleLinear::new()
      .set_domain(vec![
        (self.shortest().1 - 0.08) as f32,
        (self.longest().1 + 1.0) as f32,
      ])
      .set_range(vec![0, width - left - right]);

    let y = ScaleBand::new()
      .set_domain(
        utils::sort(&data, false)
          .iter()
          .map(|(k, _)| k.to_owned())
          .collect(),
      )
      .set_range(vec![0, height - top - bottom]);

    let view = HorizontalBarView::new()
      .set_x_scale(&x)
      .set_y_scale(&y)
      .load_data(
        &data
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
      .add_bottom_axis_label("Time")
      .save("plugins.svg")
      .unwrap();

    info!("Plot saved to `plugins.svg`");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn setup() -> (Stats, Vec<(String, f64, f64, f64)>) {
    let mut data: HashMap<String, Vec<f64>> = HashMap::new();

    let fake = vec![
      (String::from("vim-rooter"), vec![2.0, 5.2, 9.2, 10.5]),
      (String::from("vim-prettier"), vec![4.0, 5.3, 3.5, 19.2]),
      (String::from("vim-just"), vec![5.0, 2.0, 4.2, 7.8]),
      (String::from("vim-airline"), vec![6.0, 5.1, 5.2, 9.0]),
    ];

    let res = vec![
      (String::from("vim-rooter"), 6.72500, 7.2, 3.355126674210677),
      (String::from("vim-prettier"), 8.0, 4.65, 6.4996153732355575),
      (String::from("vim-just"), 4.75, 4.6, 2.0754517580517255),
      (
        String::from("vim-airline"),
        6.32500,
        5.6,
        1.5833114033569011,
      ),
    ];

    for (a, b) in fake {
      data.insert(a, b);
    }

    (Stats::new(data), res)
  }

  #[test]
  fn average() {
    let (stats, res) = setup();

    for (key, avg, _, _) in res {
      assert!(approx_eq!(f64, stats.average(&key), avg, ulps = 2));
    }
  }

  #[test]
  fn median() {
    let (stats, res) = setup();

    for (key, _, median, _) in res {
      assert!(approx_eq!(f64, stats.median(&key), median, ulps = 2));
    }
  }

  #[test]
  fn deviation() {
    let (stats, res) = setup();

    for (key, _, _, deviation) in res {
      assert!(approx_eq!(f64, stats.deviation(&key), deviation, ulps = 2));
    }
  }

  #[test]
  fn longest() {
    let (stats, _) = setup();
    assert_eq!(stats.longest(), (String::from("vim-prettier"), 8.0));
  }

  #[test]
  fn shortest() {
    let (stats, _) = setup();
    assert_eq!(stats.shortest(), (String::from("vim-just"), 4.75));
  }
}
