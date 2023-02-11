use gcs_proxy_server::Config as ServerConfig;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
}

pub async fn read_config(path: &str) -> Config {
    let f = std::fs::File::open(path).expect("Config file not found");
    serde_yaml::from_reader(f).expect("Could not parse the config")
}
