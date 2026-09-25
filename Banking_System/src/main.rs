mod auth;
mod db;
mod errors;
mod models;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("banking_system=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("Logging for banking_system")
}
