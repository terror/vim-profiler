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
    let mut sorted: Vec<(&String, &f64)> = data.iter().collect();

    if self.reversed {
      sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    } else {
      sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    }

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
        format!("{:<1$}", i + 1, sorted.len().to_string().len() + 2),
        format!("{:1$}", k, len_largest(&sorted)),
        format!("{:.1$}", v, self.prec.unwrap_or(2_usize))
      );
    }

    println!("{}", str_repeat("=", header.len()));
  }
}
