mod errors;
mod auth;
mod models;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("crypto_watchlist=debug".parse().unwrap()))
        .init();
    tracing::info!("Starting Crypto_watchlist server");
}
