use axum::http::StatusCode;
use axum::response::IntoResponse;

// health check handler
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
