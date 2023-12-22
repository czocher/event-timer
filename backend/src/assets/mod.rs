use axum::{handler::HandlerWithoutStateExt, Router};

use crate::state::AppState;

use self::http::static_handler;

mod http;

pub fn router() -> Router<AppState> {
    Router::new().fallback_service(static_handler.into_service())
}
