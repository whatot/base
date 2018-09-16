use std::default::Default;

#[derive(Clone, PartialEq, Debug)]
pub struct CorvusConfig {
    pub config_file_path: String,
    pub cluster_name: String,
    pub bind_port: u16,
    pub node_address: String,
    pub thread: u32,
    pub loglevel: u32,
    pub syslog: bool,
    pub statsd_addr: String,
    pub metric_interval: u32,
    pub stats: bool,
    pub readslave: bool,
    pub readmasterslave: bool,
    pub requirepass: String,
    pub client_timeout: u32,
    pub server_timeout: u32,
    pub bufsize: u32,
    pub slowlog_log_slower_than: i32,
    pub slowlog_max_len: u32,
    pub slowlog_statsd_enabled: bool,
}

impl Default for CorvusConfig {
    fn default() -> CorvusConfig {
        CorvusConfig {
            config_file_path: "".to_string(),
            cluster_name: "default".to_string(),
            bind_port: 12345,
            node_address: "".to_string(),
            thread: 4,
            loglevel: 2,
            syslog: false,
            statsd_addr: "localhost:8125".to_string(),
            metric_interval: 10,
            stats: false,
            readslave: false,
            readmasterslave: false,
            requirepass: "".to_string(),
            client_timeout: 0,
            server_timeout: 0,
            bufsize: 16384,
            slowlog_log_slower_than: -1,
            slowlog_max_len: 1024,
            slowlog_statsd_enabled: false,
        }
    }
}
