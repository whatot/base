extern crate structopt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Clone, PartialEq, Debug)]
#[structopt(name = "corvus-rs")]
pub struct CorvusConfig {
    #[structopt(
        name = "corvus.config",
        parse(from_os_str),
        help = "Sets a custom config file"
    )]
    pub config_file_path: PathBuf,
    #[structopt(short = "c", long = "cluster", default_value = "default")]
    pub cluster_name: String,
    #[structopt(short = "b", long = "bind", default_value = "12345")]
    pub bind_port: u16,
    #[structopt(short = "n", long = "node", default_value = "")]
    pub node_address: String,
    #[structopt(short = "t", long, default_value = "4")]
    pub thread: u32,
    #[structopt(short = "L", long, default_value = "2")]
    pub loglevel: u32,
    #[structopt(short = "l", long)]
    pub syslog: bool,
    #[structopt(long, default_value = "localhost:8125")]
    pub statsd_addr: String,
    #[structopt(long, default_value = "10")]
    pub metric_interval: u32,
    #[structopt(long)]
    pub stats: bool,
    #[structopt(long)]
    pub readslave: bool,
    #[structopt(long)]
    pub readmaster: bool,
    #[structopt(short = "P", long, default_value = "")]
    pub requirepass: String,
    #[structopt(short = "C", long, default_value = "0")]
    pub client_timeout: u32,
    #[structopt(short = "S", long, default_value = "0")]
    pub server_timeout: u32,
    #[structopt(short = "B", long, default_value = "16384")]
    pub bufsize: u32,
    #[structopt(short = "g", long, default_value = "-1")]
    pub slowlog_log_slower_than: i32,
    #[structopt(short = "G", long, default_value = "1024")]
    pub slowlog_max_len: u32,
    #[structopt(short = "E", long)]
    pub slowlog_statsd_enabled: bool,
}
