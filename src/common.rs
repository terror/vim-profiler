// stdlib
pub use std::{
  collections::HashMap,
  env,
  fmt::{self, Display, Formatter},
  fs,
  io::{self, prelude::*},
  iter,
  path::PathBuf,
  process::{Command as Cmd, Stdio},
};

// dependencies
pub use {
  charts::{Chart, HorizontalBarView, ScaleBand, ScaleLinear},
  csv::Writer,
  env_logger::{self},
  log::info,
  regex::{Regex, RegexBuilder},
  snafu::{ResultExt, Snafu},
  structopt::StructOpt,
};

// modules
pub(crate) use crate::{error, utils};

// structs and enums
pub use crate::{
  command::Command, error::Error, export::Export, opt::Opt, plugin::Plugin, plugins::Plugins,
  printer::Printer, worker::Worker,
};

// type aliases
pub type Result<T, E = Error> = std::result::Result<T, E>;

// test dependencies
#[cfg(test)]
pub use {float_cmp::approx_eq, textwrap::dedent};
