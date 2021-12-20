// stdlib
pub(crate) use std::{
  collections::HashMap,
  env,
  fmt::{self, Display, Formatter},
  fs,
  io::{self},
  iter, num,
  path::PathBuf,
  process,
  process::{Command as Cmd, Stdio},
};

// dependencies
pub(crate) use {
  charts::{Chart, HorizontalBarView, ScaleBand, ScaleLinear},
  csv::Writer,
  env_logger::{self},
  log::info,
  regex::RegexBuilder,
  snafu::{ResultExt, Snafu},
  structopt::StructOpt,
};

// structs and enums
pub(crate) use crate::{
  command::Command,
  error::{self, Error},
  opt::Opt,
  plugin::Plugin,
  plugins::Plugins,
  printer::Printer,
  worker::Worker,
};

// functions
pub(crate) use crate::{
  export::{plot, write},
  utils::repeat,
};

// type aliases
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

// test dependencies
#[cfg(test)]
pub(crate) use {float_cmp::approx_eq, textwrap::dedent};
