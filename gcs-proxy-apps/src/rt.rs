use gcs_proxy_extras::utils::get_cpu;
use tokio::runtime::{Builder, Runtime};
use tracing::info;

pub fn build() -> Runtime {
    let cores = get_cpu();

    match cores {
        // `0` is unexpected, but it's a wild world out there.
        0 | 1 => {
            info!("Using single-threaded runtime");
            Builder::new_current_thread()
                .enable_all()
                .thread_name("gcs-proxy")
                .build()
                .expect("failed to build basic runtime!")
        }
        num_cpus => {
            info!(%cores, "Using multi-threaded runtime");
            Builder::new_multi_thread()
                .enable_all()
                .thread_name("gcs-proxy")
                .worker_threads(num_cpus)
                .max_blocking_threads(num_cpus)
                .build()
                .expect("failed to build threaded runtime!")
        }
    }
}
