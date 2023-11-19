use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Form};
use chrono::Utc;
use sqlx::{self};
use uuid::Uuid;

use crate::startup::AppState;

#[derive(serde::Serialize, Debug, serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// subscriber handler
pub async fn subscribe(
    State(appstate): State<AppState>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    let connection_pool = appstate.connection_pool;

    println!("inside subscribe");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&connection_pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to Insert into Database due to {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    // println!("{form:?}");

    // format!("Welcome, {}! with {}", form.name, form.email)
}
