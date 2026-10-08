use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// The Display text is returned to the client, so it must not include the
// sheet_id or the upstream cause (reqwest errors embed the request URL).
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("tournament '{0}' not found")]
    UnknownTournament(String),
    #[error("upstream Sheets error for tournament '{tournament}'")]
    Upstream {
        tournament: String,
        sheet_id: String,
        source: anyhow::Error,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        let (status, tournament, sheet_id, cause) = match &self {
            AppError::UnknownTournament(slug) => (StatusCode::NOT_FOUND, slug.as_str(), None, None),
            AppError::Upstream {
                tournament,
                sheet_id,
                source,
            } => (
                StatusCode::BAD_GATEWAY,
                tournament.as_str(),
                Some(sheet_id.as_str()),
                Some(format!("{source:#}")),
            ),
        };
        let cause = cause.as_deref();
        if status.is_server_error() {
            tracing::error!(status = status.as_u16(), tournament, sheet_id, cause, error = %message, "request failed");
        } else {
            tracing::warn!(status = status.as_u16(), tournament, sheet_id, cause, error = %message, "request failed");
        }
        (status, Json(json!({ "error": message }))).into_response()
    }
}
