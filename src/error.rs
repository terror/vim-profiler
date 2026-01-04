use super::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(context(false), display("CSV Error: {}", source))]
  Csv { source: csv::Error },

  #[snafu(display("Invalid command: {}", cmd))]
  InvalidCommand { cmd: String },

  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },

  #[snafu(context(false), display("Parse float error: {}", source))]
  ParseFloat { source: num::ParseFloatError },

  #[snafu(display("Unable to find a vim plugin directory."))]
  PluginDirectory,

  #[snafu(display("Unable to read the contents of the `vim.log` file."))]
  ReadLog { source: io::Error },

  #[snafu(context(false), display("Regex Error: {}", source))]
  Regex { source: regex::Error },

  #[snafu(display("Unable to remove `vim.log` file."))]
  RemoveLog { source: io::Error },

  #[snafu(display("Failed to execute vim startuptime."))]
  StartupTime { source: io::Error },
}
