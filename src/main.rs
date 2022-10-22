#![allow(unused)]
use clap::Parser;
use interface::commandline_interface as cli;
pub mod core;
pub mod interface;
pub mod test;
pub fn main() {
    let args = cli::CommandlineArguments::parse();
    cli::execute_level_set_method(&args);
}
