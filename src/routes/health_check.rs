use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

// health check handler
#[tracing::instrument]
pub async fn health_check() -> impl IntoResponse {
    info!("Healthy check!");
    StatusCode::OK
}
