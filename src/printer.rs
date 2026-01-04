use crate::common::*;

#[derive(Debug)]
pub(crate) struct Printer {
  count: Option<usize>,
  prec: Option<usize>,
  reverse: bool,
}

impl Printer {
  pub fn new(reverse: bool, count: Option<usize>, prec: Option<usize>) -> Self {
    Self {
      count,
      prec,
      reverse,
    }
  }

  pub fn summary(&self, plugins: &[Plugin]) {
    let mut plugins = plugins.to_owned();

    plugins.truncate(self.count.unwrap_or(10_usize));

    let order = if self.reverse { "fastest" } else { "slowest" };
    let header = format!("Top {} {order} (n)vim plugins.", plugins.len(),);

    println!("{header}");
    println!("{}", repeat("=", header.len()));

    for (i, plugin) in plugins.iter().enumerate() {
      println!(
        "{} {} {}",
        format_args!("{:<1$}", i + 1, plugins.len().to_string().len() + 2),
        format_args!("{:1$}", plugin.name, &plugins.len_largest()),
        format_args!("{:.1$}", plugin.average(), self.prec.unwrap_or(2_usize))
      );
    }

    println!("{}", repeat("=", header.len()));
  }
}
