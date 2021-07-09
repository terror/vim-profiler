use crate::common::*;

mod command;
mod common;
mod error;
mod opt;
mod printer;
mod stats;
mod utils;
mod worker;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {}
    Err(e) => eprintln!("{}", e),
  }
}
