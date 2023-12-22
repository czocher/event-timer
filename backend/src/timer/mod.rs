use axum::{routing::get, Router};

use crate::state::AppState;

use self::http::{create_event, delete_event, get_event, list_events, update_event};

mod http;
mod models;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/events", get(list_events).post(create_event))
        .route(
            "/api/events/:event_id",
            get(get_event).put(update_event).delete(delete_event),
        )
}
