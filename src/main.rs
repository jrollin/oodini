use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use oodini::routes::{self, core::handler_404};
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "oodini=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("starting application");

    // build our application with a route
    let app = Router::new()
        .nest("/", routes::core::router().await)
        .nest("/api", routes::rest::router().await)
        .fallback(handler_404)
        .layer(TraceLayer::new_for_http());

    // add a fallback service for handling routes to unknown paths
    let (host, port) = oodini::config::from_env();
    let addr = SocketAddr::new(host.into(), port);
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {addr}");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// notify os that process will stop
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
