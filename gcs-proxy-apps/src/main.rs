mod config;
mod rt;
use gcs_proxy_extras::telemetry::{get_subscriber, init_subscriber};
use gcs_proxy_extras::utils::report_exit;
use gcs_proxy_server::Server;

#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "musl"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
fn main() {
    rt::build().block_on(async move {
        let config = config::read_config(
            &std::env::var("CONFIG_PATH").unwrap_or_else(|_| "/app/config.yaml".to_owned()),
        )
        .await;
        let subscriber = get_subscriber(
            "gcs-proxy-apps".into(),
            config.log_level.to_string(),
            std::io::stdout,
        );
        init_subscriber(subscriber);
        println!(
            "Starting {} --  version {} ",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        let server = Server::new(config.server).await.unwrap();
        let server_tasks = tokio::spawn(server.run());

        tokio::select! {
            o = server_tasks => report_exit("Server", o)
        };
    })
}
