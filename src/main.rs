#![allow(unused)]
use clap::Parser;
use interface::commandline_interface as ci;
pub mod interface;

pub fn main() {
    let args = ci::CommandlineArguments::parse();
    ci::execute_level_set_method(&args);
}
