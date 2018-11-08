extern crate structopt;
use structopt::StructOpt;

mod config;
use config::CorvusOpt;

fn main() {
    let opt = CorvusOpt::from_args();
    println!("opt: {:?}", opt);
}
