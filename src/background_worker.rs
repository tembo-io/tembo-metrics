use crate::{routes::metrics::set_and_update_metrics, state::State};
use actix_web::web;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

pub fn start_background_updater(state: web::Data<Arc<State>>) {
    tokio::spawn(async move {
        info!("Background updater started");
        let mut rng = StdRng::from_entropy();

        // Run the update immediately at startup
        info!("Performing initial metrics update");
        if let Err(e) = set_and_update_metrics(&state).await {
            error!("Error during initial metrics update: {}", e);
        } else {
            info!("Initial metrics update successful");
        }

        loop {
            // These metrics will not change much over time, so we don't need to poll often
            // Currently poll every 3-6 hours
            // Change this later if needing to pool more often
            let delay = Duration::from_secs(rng.gen_range(10800..=21600));
            info!(
                "Sleeping for {} seconds before next update",
                delay.as_secs()
            );
            time::sleep(delay).await;

            if !state.updating.load(std::sync::atomic::Ordering::Relaxed) {
                state
                    .updating
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                info!("Starting metrics update");

                match set_and_update_metrics(&state).await {
                    Ok(_) => info!("Metrics updated successfully"),
                    Err(e) => error!("Error updating metrics: {}", e),
                }

                *state.last_update.lock().await = std::time::Instant::now();
                state
                    .updating
                    .store(false, std::sync::atomic::Ordering::Relaxed);
            } else {
                info!("Update already in progress, skipping");
            }
        }
    });
    info!("Background updater task spawned");
}
