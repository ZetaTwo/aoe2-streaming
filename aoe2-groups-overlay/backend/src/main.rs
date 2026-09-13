use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

mod config;
mod error;
mod parse;
mod routes;
mod sheets;

use crate::{config::Config, routes::AppState, sheets::SheetsClient};

#[tokio::main]
async fn main() -> Result<()> {
    // rustls 0.23 may have multiple CryptoProviders compiled in (ring + aws-lc-rs)
    // through transitive deps. Pin one before any TLS handshake happens.
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring CryptoProvider");

    registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(
            fmt::layer()
                .json()
                .flatten_event(true)
                .with_current_span(true)
                .with_span_list(true)
                .with_writer(std::io::stdout),
        )
        .init();

    let tournaments_path = std::env::var_os("TOURNAMENTS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("tournaments.toml"));
    let config_path = std::env::var_os("CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.toml"));
    let sheet_ids_path = std::env::var_os("SHEET_IDS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/etc/aoe2-groups-proxy/sheet-ids.toml"));
    tracing::info!(
        tournaments_path = %tournaments_path.display(),
        config_path = %config_path.display(),
        sheet_ids_path = %sheet_ids_path.display(),
        "loading configuration"
    );
    let config =
        Config::load(&config_path, &tournaments_path, &sheet_ids_path).context("loading config")?;
    tracing::info!(
        tournament_count = config.tournaments.len(),
        "loaded configuration"
    );

    let sheets = SheetsClient::new()
        .await
        .context("constructing Sheets client")?;

    let bind_addr = config.server.bind_addr.clone();
    let port = config.server.port;
    let allowed = config.server.allowed_origins.clone();
    let state = Arc::new(AppState { config, sheets });
    let app = routes::router(state);

    let listener = TcpListener::bind((bind_addr.as_str(), port))
        .await
        .with_context(|| format!("binding to {bind_addr}:{port}"))?;
    tracing::info!(%bind_addr, port, ?allowed, "listening for requests");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Axum server error")?;
    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c().await.expect("install ctrl-c handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => tracing::info!("received SIGINT, shutting down"),
        _ = terminate => tracing::info!("received SIGTERM, shutting down"),
    }
}
