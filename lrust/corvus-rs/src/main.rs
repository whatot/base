extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Corvus-rs")
        .version("0.1")
        .author("whatot whatot2@gmail.com")
        .about("A lightweight redis cluster proxy")
        .arg(
            Arg::with_name("config")
                .index(1)
                .value_name("corvus.conf")
                .help("Sets a custom config file")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("thread")
                .short("t")
                .long("thread")
                .default_value("4")
                .takes_value(true),
        ).arg(
            Arg::with_name("node")
                .short("n")
                .long("node")
                .takes_value(true),
        ).arg(
            Arg::with_name("cluster")
                .short("c")
                .long("cluster")
                .takes_value(true),
        ).arg(
            Arg::with_name("bind")
                .short("b")
                .long("bind")
                .takes_value(true),
        ).arg(Arg::with_name("syslog").short("l").long("syslog"))
        .arg(
            Arg::with_name("read-strategy")
                .short("s")
                .long("read-strategy")
                .possible_values(&["master", "read-slave-only", "both"])
                .default_value("master")
                .takes_value(true),
        ).arg(
            Arg::with_name("bufsize")
                .short("B")
                .long("bufsize")
                .default_value("16384")
                .takes_value(true),
        ).arg(
            Arg::with_name("client_timeout")
                .short("C")
                .long("client_timeout")
                .takes_value(true),
        ).arg(
            Arg::with_name("server_timeout")
                .short("S")
                .long("server_timeout")
                .takes_value(true),
        ).arg(
            Arg::with_name("statsd")
                .short("A")
                .long("statsd")
                .takes_value(true),
        ).arg(
            Arg::with_name("metric_interval")
                .short("m")
                .long("metric_interval")
                .takes_value(true),
        ).arg(
            Arg::with_name("loglevel")
                .short("L")
                .long("loglevel")
                .takes_value(true),
        ).arg(
            Arg::with_name("requirepass")
                .short("P")
                .long("requirepass")
                .empty_values(true)
                .possible_value("password")
                .takes_value(true),
        ).arg(
            Arg::with_name("slowlog-log-slower-than")
                .short("g")
                .long("slowlog-log-slower-than")
                .takes_value(true),
        ).arg(
            Arg::with_name("slowlog-max-len")
                .short("G")
                .long("slowlog-max-len")
                .takes_value(true),
        ).arg(
            Arg::with_name("slowlog-statsd-enabled")
                .short("E")
                .long("slowlog-statsd-enabled")
                .possible_values(&["0", "1"])
                .default_value("0")
                .takes_value(true),
        ).get_matches();
    println!("match: {:?}", matches);
}
