use crate::application::orchestrators::snapshot_from_spans;
use crate::domain::contracts::TraceRepository;
use crate::domain::entities::{ExecutionSnapshot, LensSpan, TraceSummary};
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse};
use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;

// Built by the React/TypeScript app in `crates/or-lens/dashboard/`
// (`npm run build` regenerates these; the output is committed so cargo
// consumers never need Node).
const DASHBOARD_HTML: &str = include_str!("../../assets/dist/index.html");
const DASHBOARD_CSS: &str = include_str!("../../assets/dist/dashboard.css");
const DASHBOARD_JS: &str = include_str!("../../assets/dist/dashboard.js");

#[derive(Clone)]
pub(crate) struct AppState<R: TraceRepository> {
    pub(crate) repository: R,
}

pub(crate) fn router<R: TraceRepository>(repository: R) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/assets/dashboard.css", get(styles))
        .route("/assets/dashboard.js", get(script))
        .route("/api/traces", get(list_traces::<R>))
        .route("/api/traces/{trace_id}", get(get_trace::<R>))
        .route("/api/spans", post(ingest_spans::<R>))
        .with_state(AppState { repository })
}

async fn index() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

async fn styles() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        DASHBOARD_CSS,
    )
}

async fn script() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        DASHBOARD_JS,
    )
}

/// Accepts either a single span object or an array of spans, so external
/// processes (agents in any language) can push traces into the dashboard.
#[derive(Deserialize)]
#[serde(untagged)]
enum SpanIngest {
    One(Box<LensSpan>),
    Many(Vec<LensSpan>),
}

async fn ingest_spans<R: TraceRepository>(
    State(state): State<AppState<R>>,
    Json(payload): Json<SpanIngest>,
) -> impl IntoResponse {
    let spans = match payload {
        SpanIngest::One(span) => vec![*span],
        SpanIngest::Many(spans) => spans,
    };
    for span in &spans {
        if span.trace_id.trim().is_empty() || span.span_id.trim().is_empty() {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({
                    "error": "every span must set a non-empty trace_id and span_id"
                })),
            );
        }
    }
    let accepted = spans.len();
    for span in spans {
        state.repository.record_span(span);
    }
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({ "accepted": accepted })),
    )
}

async fn list_traces<R: TraceRepository>(
    State(state): State<AppState<R>>,
) -> Json<Vec<TraceSummary>> {
    Json(state.repository.traces())
}

async fn get_trace<R: TraceRepository>(
    Path(trace_id): Path<String>,
    State(state): State<AppState<R>>,
) -> Result<Json<ExecutionSnapshot>, impl IntoResponse> {
    state
        .repository
        .trace(&trace_id)
        .map(|spans| Json(snapshot_from_spans(&trace_id, &spans)))
        .ok_or((StatusCode::NOT_FOUND, "trace not found"))
}
