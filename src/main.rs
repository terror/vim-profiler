use {
  arguments::Arguments,
  charts::{Chart, HorizontalBarView, ScaleBand, ScaleLinear},
  clap::Parser,
  command::Command,
  csv::Writer,
  env_logger::{self},
  error::Error,
  export::{plot, write},
  log::info,
  num_traits::cast::ToPrimitive,
  plugin::Plugin,
  plugins::Plugins,
  printer::Printer,
  regex::RegexBuilder,
  snafu::{ResultExt, Snafu},
  std::{
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
  },
  utils::repeat,
  worker::Worker,
};

mod arguments;
mod command;
mod error;
mod export;
mod plugin;
mod plugins;
mod printer;
mod utils;
mod worker;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
pub(crate) use {float_cmp::approx_eq, textwrap::dedent};

fn main() {
  match Arguments::parse().run() {
    Ok(()) => {}
    Err(e) => {
      eprintln!("{e}");
      process::exit(1);
    }
  }
}
