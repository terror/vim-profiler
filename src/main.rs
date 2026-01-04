use crate::common::*;

mod arguments;
mod command;
mod common;
mod error;
mod export;
mod plugin;
mod plugins;
mod printer;
mod utils;
mod worker;

fn main() {
  match Arguments::parse().run() {
    Ok(()) => {}
    Err(e) => {
      eprintln!("{e}");
      process::exit(1);
    }
  }
}
