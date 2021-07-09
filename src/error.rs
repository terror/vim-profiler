use crate::common::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Invalid command: {}", cmd))]
  InvalidCommand { cmd: String },

  #[snafu(display("Failed to execute vim startuptime."))]
  StartupTime { source: io::Error },

  #[snafu(display("Unable to read the contents of the `vim.log` file."))]
  ReadLog { source: io::Error },

  #[snafu(display("Unable to remove `vim.log` file."))]
  RemoveLog { source: io::Error },

  #[snafu(display("Unable to find a vim plugin directory."))]
  PluginDirectory,
}
