use crate::common::*;

pub(crate) fn write(path: PathBuf, plugins: &[Plugin]) -> Result<(), Error> {
  let mut writer = Writer::from_path(path.clone())?;

  writer.write_record(&["Plugin", "Max", "Min", "Median", "Average", "Deviation"])?;

  for plugin in plugins.iter() {
    writer.write_record(&[
      plugin.name.to_owned(),
      format!("{:.5}", plugin.max()),
      format!("{:.5}", plugin.min()),
      format!("{:.5}", plugin.median()),
      format!("{:.5}", plugin.average()),
      format!("{:.5}", plugin.deviation()),
    ])?;
  }

  writer.flush()?;

  info!("Statistics written to `{}`", path.display());

  Ok(())
}

pub(crate) fn plot(path: PathBuf, plugins: &[Plugin]) -> Result<(), Error> {
  let plugins = plugins.to_vec();

  let (width, height, top, right, bottom, left) = (1200, 800, 90, 10, 50, 120);

  let x = ScaleLinear::new()
    .set_domain(vec![
      (plugins.min() - 0.05) as f32,
      (plugins.max() + 1.0) as f32,
    ])
    .set_range(vec![0, width - left - right]);

  let y = ScaleBand::new()
    .set_domain(
      plugins
        .iter()
        .map(|plugin| plugin.name.to_owned())
        .collect(),
    )
    .set_range(vec![0, height - top - bottom]);

  let view = HorizontalBarView::new()
    .set_x_scale(&x)
    .set_y_scale(&y)
    .load_data(
      &plugins
        .iter()
        .map(|plugin| (plugin.name.to_owned(), plugin.average() as f32))
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
    .save(path.clone())
    .unwrap();

  info!("Plot saved to `{}`", path.display());

  Ok(())
}
