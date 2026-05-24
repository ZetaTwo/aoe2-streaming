use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderValue, Method},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::{
    config::Config,
    error::AppError,
    parse::{parse_tournament, BatchGetResponse, BracketStanding},
    sheets::SheetsClient,
};

pub struct AppState {
    pub config: Config,
    pub sheets: SheetsClient,
}

#[derive(Serialize)]
pub struct TournamentResponse {
    pub brackets: Vec<BracketStanding>,
}

pub fn router(state: Arc<AppState>) -> Router {
    let cors = build_cors(&state.config.server.allowed_origins);
    Router::new()
        .route("/healthz", get(healthz))
        .route("/tournaments/:slug", get(get_tournament))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

async fn healthz() -> &'static str {
    "OK"
}

fn build_cors(origins: &[String]) -> CorsLayer {
    let base = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers(Any);
    if origins.is_empty() {
        return base.allow_origin(Any);
    }
    let parsed: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|o| HeaderValue::from_str(o).ok())
        .collect();
    if parsed.is_empty() {
        tracing::warn!("no allowed_origins parsed as valid HeaderValue; falling back to Any");
        return base.allow_origin(Any);
    }
    base.allow_origin(parsed)
}

async fn get_tournament(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Json<TournamentResponse>, AppError> {
    let tournament = state
        .config
        .tournaments
        .get(&slug)
        .ok_or_else(|| AppError::UnknownTournament(slug.clone()))?;

    let ranges: Vec<String> = tournament
        .brackets
        .iter()
        .flat_map(|b| {
            b.group_ranges
                .iter()
                .map(move |r| format!("'{}'!{}", b.name, r))
        })
        .collect();

    // Placeholder tournaments (no brackets yet) short-circuit to an empty
    // response without consulting Sheets — useful while sheet_id is "TBD".
    let response = if ranges.is_empty() {
        BatchGetResponse {
            value_ranges: Vec::new(),
        }
    } else {
        state
            .sheets
            .batch_get(&tournament.sheet_id, &ranges)
            .await?
    };

    let brackets = parse_tournament(tournament, &response.value_ranges);
    Ok(Json(TournamentResponse { brackets }))
}
