use gcs_proxy_deps::cloud_storage::GcsConfig;
use gcs_proxy_extras::utils::get_cpu;
use getset::Getters;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub http: HttpServerConfig,
    pub auth: AuthConfig,
    pub gcs: GcsConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Getters, Clone)]
pub struct HttpServerConfig {
    #[getset(get = "pub with_prefix")]
    port: u32,
    #[getset(get = "pub with_prefix")]
    shutdown_timeout: u64,
    #[getset(get = "pub with_prefix")]
    #[serde(default = "get_cpu")]
    worker_total: usize,
}
