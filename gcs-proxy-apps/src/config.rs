use gcs_proxy_server::Config as ServerConfig;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub log_level: LogLevel,
    pub server: ServerConfig,
}

#[derive(Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum LogLevel {
    info,
    warn,
    debug,
    error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LogLevel::debug => write!(f, "debug"),
            LogLevel::error => write!(f, "error"),
            LogLevel::info => write!(f, "info"),
            LogLevel::warn => write!(f, "warn"),
        }
    }
}

pub async fn read_config(path: &str) -> Config {
    let f = std::fs::File::open(path).expect("Config file not found");
    serde_yaml::from_reader(f).expect("Could not parse the config")
}
