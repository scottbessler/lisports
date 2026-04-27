use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::render;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Cache(String),
    NotFound(String),
    Parse(String),
    Upstream(String),
}

impl AppError {
    pub fn upstream(err: impl std::fmt::Display) -> Self {
        Self::Upstream(err.to_string())
    }

    pub fn cache(err: impl std::fmt::Display) -> Self {
        Self::Cache(err.to_string())
    }

    pub fn parse(err: impl std::fmt::Display) -> Self {
        Self::Parse(err.to_string())
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Cache(_) | Self::Parse(_) | Self::Upstream(_) => StatusCode::BAD_GATEWAY,
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "Bad request",
            Self::NotFound(_) => "Not found",
            Self::Cache(_) => "Cache error",
            Self::Parse(_) => "Could not parse upstream data",
            Self::Upstream(_) => "Could not load upstream data",
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::BadRequest(message)
            | Self::Cache(message)
            | Self::NotFound(message)
            | Self::Parse(message)
            | Self::Upstream(message) => message,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status(),
            Html(render::error_page(self.title(), self.message())),
        )
            .into_response()
    }
}
