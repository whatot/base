use structopt::StructOpt;

mod config;
use crate::config::CorvusOpt;

fn main() {
    let opt = CorvusOpt::from_args();
    println!("opt: {:?}", opt);
}
