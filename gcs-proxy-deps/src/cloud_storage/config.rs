use getset::Getters;
use serde::Deserialize;

#[derive(Deserialize, Getters, Clone)]
pub struct GcsConfig {
    #[getset(get = "pub with_prefix")]
    bucket: String,
    #[getset(get = "pub with_prefix")]
    folder: Option<String>,
    #[getset(get = "pub with_prefix")]
    service_account_b64: Option<String>,
}
