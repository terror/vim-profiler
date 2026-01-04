use super::*;

#[derive(Debug, Parser)]
#[command(name = "vim-profiler", about = "A vim profiling tool.")]
pub(crate) struct Arguments {
  #[arg(short, long, default_value = "vim")]
  /// The command to run, e.g vim or neovim.
  command: Command,
  #[arg(short = 'n', long)]
  /// The number of plugins to list in the output.
  count: Option<usize>,
  #[arg(short, long)]
  /// Export the results to a CSV file.
  export: Option<PathBuf>,
  #[arg(short, long)]
  /// A file to open
  file: Option<PathBuf>,
  #[arg(short, long)]
  /// The number of iterations.
  iter: Option<i64>,
  #[arg(short, long)]
  /// Plot the data and save it to a SVG file
  plot: Option<PathBuf>,
  #[arg(short = 'x', long)]
  /// Precision in the output.
  precision: Option<usize>,
  #[arg(short, long)]
  /// Display the plugin times in reverse order (fastest first).
  reverse: bool,
  #[arg(short, long)]
  /// Show system plugins in the output.
  sys: bool,
  #[arg(short, long)]
  /// Add informative messages during program execution.
  verbose: bool,
}

impl Arguments {
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
