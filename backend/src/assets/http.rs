use crate::{error::Error, error::Result};
use axum::{
    http::{header, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::{EmbeddedFile, RustEmbed};

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../frontend/dist/spa"]
struct Assets;

pub async fn static_handler(uri: Uri) -> Result<Response> {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    if let Some(content) = Assets::get(path) {
        Ok(content.into_response())
    } else if path.contains('.') {
        return Err(Error::NotFound);
    } else {
        index_html().await
    }
}

#[allow(clippy::unused_async)]
async fn index_html() -> Result<Response> {
    let content = Assets::get(INDEX_HTML).ok_or(Error::NotFound)?;
    Ok(content.into_response())
}

trait EmbeddedFileEx {
    fn into_response(self) -> Response;
}

impl EmbeddedFileEx for EmbeddedFile {
    fn into_response(self) -> Response {
        (
            [(header::CONTENT_TYPE, self.metadata.mimetype())],
            self.data,
        )
            .into_response()
    }
}
