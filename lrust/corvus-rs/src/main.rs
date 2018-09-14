extern crate clap;
use clap::{App, Arg};

fn main() {
    let _matches = App::new("Corvus-rs")
        .version("0.1")
        .author("whatot whatot2@gmail.com")
        .about("A lightweight redis cluster proxy")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        ).get_matches();
}
