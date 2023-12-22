use crate::{error::Result, timer::models::Event};
use axum::{extract::Path, Json};

pub async fn list_events() -> Result<Json<Vec<Event>>> {
    todo!()
}

pub async fn create_event() -> Result<Json<Vec<Event>>> {
    todo!()
}

pub async fn get_event(Path(event_id): Path<i32>) -> Result<()> {
    todo!()
}

pub async fn delete_event(Path(event_id): Path<i32>) -> Result<()> {
    todo!()
}

pub async fn update_event(Path(event_id): Path<i32>) -> Result<()> {
    todo!()
}
