pub(crate) use std::{
  collections::HashMap,
  convert::TryFrom,
  env,
  fmt::{self, Display, Formatter},
  fs,
  io::{self},
  iter, num,
  path::PathBuf,
  process,
  process::{Command as Cmd, Stdio},
};

pub(crate) use {
  charts::{Chart, HorizontalBarView, ScaleBand, ScaleLinear},
  clap::Parser,
  csv::Writer,
  env_logger::{self},
  log::info,
  regex::RegexBuilder,
  snafu::{ResultExt, Snafu},
};

pub(crate) use crate::{
  arguments::Arguments,
  command::Command,
  error::{self, Error},
  plugin::Plugin,
  plugins::Plugins,
  printer::Printer,
  worker::Worker,
};

pub(crate) use crate::{
  export::{plot, write},
  utils::repeat,
};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
pub(crate) use {float_cmp::approx_eq, textwrap::dedent};
