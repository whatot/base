use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, PartialEq, Debug)]
#[command(name = "corvus-rs")]
pub struct CorvusOpt {
    #[arg(value_name = "corvus.config", help = "Sets a custom config file")]
    pub config_file_path: PathBuf,
    #[arg(short = 'c', long = "cluster", default_value = "default")]
    pub cluster_name: String,
    #[arg(short = 'b', long = "bind", default_value = "12345")]
    pub bind_port: u16,
    #[arg(short = 'n', long = "node", default_value = "")]
    pub node_address: String,
    #[arg(short = 't', long, default_value = "4")]
    pub thread: u32,
    #[arg(short = 'L', long, default_value = "2")]
    pub loglevel: u32,
    #[arg(short = 'l', long)]
    pub syslog: bool,
    #[arg(long, default_value = "localhost:8125")]
    pub statsd_addr: String,
    #[arg(long, default_value = "10")]
    pub metric_interval: u32,
    #[arg(long)]
    pub stats: bool,
    #[arg(long)]
    pub readslave: bool,
    #[arg(long)]
    pub readmaster: bool,
    #[arg(short = 'P', long, default_value = "")]
    pub requirepass: String,
    #[arg(short = 'C', long, default_value = "0")]
    pub client_timeout: u32,
    #[arg(short = 'S', long, default_value = "0")]
    pub server_timeout: u32,
    #[arg(short = 'B', long, default_value = "16384")]
    pub bufsize: u32,
    #[arg(short = 'g', long, default_value = "-1")]
    pub slowlog_log_slower_than: i32,
    #[arg(short = 'G', long, default_value = "1024")]
    pub slowlog_max_len: u32,
    #[arg(short = 'E', long)]
    pub slowlog_statsd_enabled: bool,
}
