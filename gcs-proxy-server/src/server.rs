use super::route::download_async;
use crate::config::Config;
use actix_web::dev::Server as http_server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use anyhow::Error;
use gcs_proxy_deps::cloud_storage::gcs::GcsClient;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Server {
    server: http_server,
}

impl Server {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let address = format!("0.0.0.0:{}", config.http.get_port());
        let listener = TcpListener::bind(address)?;
        let gcs_instance = GcsClient::new(config.gcs)?;
        let auth = HttpAuthentication::basic(super::middleware::validator);
        let server = HttpServer::new(move || {
            App::new()
                .app_data(config.auth.clone())
                .wrap(TracingLogger::default())
                .wrap(auth.clone())
                .app_data(Data::new(gcs_instance.clone()))
                .service(download_async)
        })
        .listen(listener)?
        .shutdown_timeout(*config.http.get_shutdown_timeout())
        .workers(*config.http.get_worker_total())
        .run();
        Ok(Self { server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
