use axum::{response::IntoResponse, Form};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// subscriber handler
pub async fn subscribe(Form(form): Form<FormData>) -> impl IntoResponse {
    format!("Welcome, {}! with {}", form.name, form.email)
}
