use clap::Parser;

mod config;
use crate::config::CorvusOpt;

fn main() {
    let opt = CorvusOpt::parse();
    println!("opt: {:?}", opt);
}
