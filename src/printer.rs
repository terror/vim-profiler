use crate::common::*;

#[derive(Debug)]
pub struct Printer {
  reverse: bool,
  count:   Option<usize>,
  prec:    Option<usize>,
}

impl Printer {
  pub fn new(reverse: bool, count: Option<usize>, prec: Option<usize>) -> Self {
    Self {
      reverse,
      count,
      prec,
    }
  }

  pub fn summary(&self, data: HashMap<String, f64>) {
    let mut data = utils::sort(&data, self.reverse);

    data.truncate(self.count.unwrap_or(10_usize));

    let header = format!(
      "Top {} {} (n)vim plugins.",
      data.len(),
      if self.reverse { "fastest" } else { "slowest" }
    );

    println!("{}", header);
    println!("{}", utils::repeat("=", header.len()));

    for (i, (k, v)) in data.iter().enumerate() {
      println!(
        "{} {} {:10}",
        format!("{:<1$}", i + 1, data.len().to_string().len() + 2),
        format!("{:1$}", k, utils::len_largest(&data)),
        format!("{:.1$}", v, self.prec.unwrap_or(2_usize))
      );
    }

    println!("{}", utils::repeat("=", header.len()));
  }
}
