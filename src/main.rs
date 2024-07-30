use actix_web::{dev::ServerHandle, web, App, HttpServer};
use clap::Parser;
use parking_lot::Mutex;
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Address to bind the metrics server to
    #[arg(long, default_value = "127.0.0.1")]
    server_host: String,

    /// Port to bind the metrics server to
    #[arg(long, default_value = "8080")]
    server_port: u16,

    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// AWs Region to use (default: us-east-1)
    #[arg(short, long, default_value = "us-east-1")]
    region: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // Initialize logger
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tembo_metrics::get_log_filter(&cli.log_level))
        .with_thread_ids(false)
        .init();

    // Set up AppState and AWS SDK clients
    let state = web::Data::new(tembo_metrics::state::State::new(cli.region).await);

    info!(
        "Starting HTTP server at http://{}:{}/",
        cli.server_host, cli.server_port
    );

    let stop_handle = web::Data::new(StopHandle::default());

    // Start background worker to poll AWS services
    tembo_metrics::background_worker::start_background_updater(state.clone());

    // Build actix-web server
    let server = HttpServer::new({
        let stop_handle = stop_handle.clone();
        move || {
            App::new()
                .app_data(state.clone())
                .app_data(stop_handle.clone())
                .service(tembo_metrics::routes::metrics::metrics)
                .service(tembo_metrics::routes::health::liveness)
                .service(tembo_metrics::routes::health::readiness)
        }
    })
    .bind((cli.server_host, cli.server_port))?
    .shutdown_timeout(5)
    .run();

    stop_handle.register(server.handle());

    server.await
}

#[derive(Default)]
struct StopHandle {
    inner: Mutex<Option<ServerHandle>>,
}

impl StopHandle {
    // Set the ServerHandle to stop
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }
}
