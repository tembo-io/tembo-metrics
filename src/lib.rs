pub mod aws;
pub mod background_worker;
pub mod metrics;
pub mod routes;
pub mod state;

pub fn get_log_filter(log_level: &str) -> tracing_subscriber::EnvFilter {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    tracing_subscriber::EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy()
}
