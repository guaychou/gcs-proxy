use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use tracing::{debug, warn};

pub fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}

pub fn get_cpu() -> usize {
    let mut cores = std::env::var("NODE_WATCHER_CORES")
        .ok()
        .and_then(|v| {
            let opt = v.parse::<usize>().ok().filter(|n| *n > 0);
            if opt.is_none() {
                warn!(NODE_WATCHER_CORES = %v, "Ignoring invalid configuration");
            }
            opt
        })
        .unwrap_or_else(num_cpus::get);

    let cpus = num_cpus::get();
    debug!("Available cpu, {}", cpus);
    debug_assert!(cpus > 0, "At least one CPU must be available");
    if cores > cpus {
        warn!(
            cpus,
            NODE_WATCHER_CORES = cores,
            "Ignoring configuration due to insufficient resources"
        );
        cores = cpus;
    }
    cores
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test without any env set
    #[test]
    fn cpu_ci_works() {
        let result = get_cpu();
        assert_eq!(result, num_cpus::get());
    }
}
