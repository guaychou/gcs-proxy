use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Response {
    #[serde(rename(serialize = "apiVersion"))]
    api_version: String,
    success: bool,
}

impl Response {
    pub fn success() -> Self {
        Self {
            api_version: String::from("v1"),
            success: true,
        }
    }
}

#[derive(Deserialize)]
pub struct RequestModel {
    pub filename: String,
}
