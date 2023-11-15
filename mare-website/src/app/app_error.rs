use askama::Template;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Template)]
#[template(path = "error.askama.html")]
struct ErrorTemplate {
    code: hyper::StatusCode,
    source: anyhow::Error,
}

pub(crate) struct AppError {
    pub(crate) code: hyper::StatusCode,
    pub(crate) source: anyhow::Error,
}

impl AppError {
    pub(crate) fn new(code: hyper::StatusCode, source: anyhow::Error) -> Self {
        Self { source, code }
    }

    pub(crate) fn with_status_404(source: anyhow::Error) -> Self {
        Self {
            code: hyper::StatusCode::NOT_FOUND,
            source,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        ErrorTemplate {
            code: self.code,
            source: self.source,
        }
        .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            code: hyper::StatusCode::NOT_FOUND,
            source: err.into(),
        }
    }
}
