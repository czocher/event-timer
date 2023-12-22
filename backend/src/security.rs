use axum::http::{HeaderValue, Method};

use tower_http::cors::CorsLayer;
use tracing_unwrap::ResultExt;

use crate::cli::Args;

pub fn create_cors_layer(args: &Args) -> CorsLayer {
    if cfg!(debug_assertions) {
        CorsLayer::very_permissive()
    } else {
        let allowed_origin = args.allowed_origin.parse::<HeaderValue>().unwrap_or_log();

        CorsLayer::new()
            .allow_credentials(true)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::OPTIONS])
            .allow_origin(allowed_origin)
    }
}
