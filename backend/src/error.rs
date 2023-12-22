use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::borrow::Cow;
use std::collections::HashMap;
use tracing::log::error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized,

    #[error("request path not found")]
    NotFound,

    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    #[error("request path not found")]
    MultipartParse(#[from] MultipartError),

    #[error("an internal server error occurred")]
    Eyre(#[from] eyre::Error),

    #[error("an internal server error occurred")]
    Serde(#[from] serde_json::Error),
}

impl Error {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key.
    ///
    /// Try "Go to Usage" in an IDE for examples.
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } | Self::MultipartParse { .. } => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            Self::Eyre(_) | Self::Serde(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }

                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            }
            Error::Eyre(ref err) => error!("Eyre error: {:?}", err),
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}
