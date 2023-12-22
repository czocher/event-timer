use axum::Router;
use clap::Parser;
use eyre::Context;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{cli::Args, security::create_cors_layer, state::AppState};

mod assets;
mod cli;
mod error;
mod security;
mod state;
mod timer;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    if std::env::var("NO_COLOR") == Err(std::env::VarError::NotPresent) {
        color_eyre::install()?;
        tracing_subscriber::fmt::init();
    } else {
        tracing_subscriber::fmt().with_ansi(false).init();
    }

    let args = Args::parse();

    serve(&args).await?;

    Ok(())
}

#[allow(clippy::missing_errors_doc)]
pub async fn serve(args: &Args) -> eyre::Result<()> {
    let app = Router::new()
        .merge(api_router())
        .layer(create_cors_layer(args))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {});

    info!("Server started at http://{}", args.bind);

    let listener = tokio::net::TcpListener::bind(&args.bind).await?;
    axum::serve(listener, app.into_make_service())
        .await
        .wrap_err("error running HTTP server")
}

fn api_router() -> Router<AppState> {
    assets::router()
}
