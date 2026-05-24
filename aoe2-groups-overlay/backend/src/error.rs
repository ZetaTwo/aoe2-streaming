use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("tournament '{0}' not found")]
    UnknownTournament(String),
    #[error("upstream Sheets error: {0:#}")]
    Upstream(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::UnknownTournament(_) => StatusCode::NOT_FOUND,
            AppError::Upstream(_) => StatusCode::BAD_GATEWAY,
        };
        let message = self.to_string();
        if status.is_server_error() {
            tracing::error!("{message}");
        } else {
            tracing::warn!("{message}");
        }
        (status, Json(json!({ "error": message }))).into_response()
    }
}
