extern crate structopt;
use structopt::StructOpt;

mod config;
use config::CorvusConfig;

fn main() {
    let config = CorvusConfig::from_args();
    println!("match: {:?}", config);
}
