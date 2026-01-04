use crate::common::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "vim-profiler", about = "A vim profiling tool.")]
pub(crate) struct Opt {
  #[structopt(short, long, parse(try_from_str = Command::parse), default_value = "vim")]
  /// The command to run, e.g vim or neovim.
  command: Command,
  #[structopt(short = "n", long)]
  /// The number of plugins to list in the output.
  count: Option<usize>,
  #[structopt(short, long)]
  /// Export the results to a CSV file.
  export: Option<PathBuf>,
  #[structopt(short, long)]
  /// A file to open
  file: Option<PathBuf>,
  #[structopt(short, long)]
  /// The number of iterations.
  iter: Option<i64>,
  #[structopt(short, long)]
  /// Plot the data and save it to a SVG file
  plot: Option<PathBuf>,
  #[structopt(short = "x", long)]
  /// Precision in the output.
  precision: Option<usize>,
  #[structopt(short, long)]
  /// Display the plugin times in reverse order (fastest first).
  reverse: bool,
  #[structopt(short, long)]
  /// Show system plugins in the output.
  sys: bool,
  #[structopt(short, long)]
  /// Add informative messages during program execution.
  verbose: bool,
}

impl Opt {
  pub fn run(self) -> Result<()> {
    if self.verbose {
      // SAFETY: This is called at program startup before any other threads are spawned.
      unsafe { env::set_var("RUST_LOG", "info") };
    }

    env_logger::init();
    info!("Starting run ...");

    let plugins =
      Worker::new(self.command, self.iter.unwrap_or(1), self.sys, self.file)
        .run()?
        .sort(self.reverse);

    if self.export.is_none() && self.plot.is_none() {
      Printer::new(self.reverse, self.count, self.precision).summary(&plugins);
      return Ok(());
    }

    if let Some(path) = self.export {
      info!("Writing statistics to CSV file ...");
      write(path, &plugins)?;
    }

    if let Some(path) = self.plot {
      info!("Plotting statistics ...");
      plot(path, &plugins)?;
    }

    Ok(())
  }
}
