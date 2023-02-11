use super::config::GcsConfig;
use base64::{engine::general_purpose, Engine as _};
use cloud_storage::Client;
use cloud_storage::Error;
use futures::stream::Stream;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GcsClient {
    client: Arc<Client>,
    bucket: String,
    folder: Option<String>,
}

impl GcsClient {
    pub fn new(config: GcsConfig) -> Result<GcsClient, anyhow::Error> {
        let sa =
            match std::env::var("GCP_SA_B64") {
                Ok(val) => val,
                Err(e) => match config.get_service_account_b64().to_owned() {
                    Some(val) => val,
                    None => return Err(anyhow::Error::new(e).context(
                        "neither GCP_SA_B64 env variable nor service account in config file is set",
                    )),
                },
            };

        let data = String::from_utf8(general_purpose::STANDARD.decode(sa)?)?;
        std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS_JSON", data);
        let client = Arc::new(Client::new());
        Ok(GcsClient {
            client,
            bucket: config.get_bucket().to_string(),
            folder: config.get_folder().to_owned(),
        })
    }

    pub async fn get_object(&self, file_name: &str) -> Result<Vec<u8>, Error> {
        let file_name = if let Some(prefix) = &self.folder {
            format!("{prefix}/{file_name}")
        } else {
            file_name.to_string()
        };
        self.client
            .object()
            .download(self.bucket.as_str(), file_name.as_str())
            .await
    }

    pub async fn get_object_stream(
        &self,
        file_name: &str,
    ) -> Result<impl Stream<Item = Result<u8, Error>> + Unpin, Error> {
        let file_name = if let Some(prefix) = &self.folder {
            format!("{prefix}/{file_name}")
        } else {
            file_name.to_string()
        };
        self.client
            .object()
            .download_streamed(self.bucket.as_str(), file_name.as_str())
            .await
    }
}
