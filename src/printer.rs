use crate::common::*;

pub struct Printer {
  reversed: bool,
  count:    Option<usize>,
  prec:     Option<usize>,
}

impl Printer {
  pub fn new(reversed: bool, count: Option<usize>, prec: Option<usize>) -> Self {
    Self {
      reversed,
      count,
      prec,
    }
  }

  pub fn summary(&self, data: &HashMap<String, f64>) {
    let mut sorted = sort_times(data, self.reversed);

    sorted.truncate(self.count.unwrap_or(10_usize));

    let header = format!(
      "Top {} {} (n)vim plugins.",
      sorted.len(),
      if self.reversed { "fastest" } else { "slowest" }
    );

    println!("{}", header);
    println!("{}", str_repeat("=", header.len()));

    for (i, (k, v)) in sorted.iter().enumerate() {
      println!(
        "{} {} {:10}",
        format!("{:<1$}", i + 1, sorted.clone().len().to_string().len() + 2),
        format!("{:1$}", k, len_largest(sorted.clone())),
        format!("{:.1$}", v, self.prec.unwrap_or(2_usize))
      );
    }

    println!("{}", str_repeat("=", header.len()));
  }
}
