use crate::common::*;

mod command;
mod common;
mod error;
mod export;
mod opt;
mod plugin;
mod plugins;
mod printer;
mod utils;
mod worker;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {}
    Err(e) => {
      eprintln!("{}", e);
      process::exit(1);
    }
  }
}
