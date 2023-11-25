use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Form};
use chrono::Utc;
use sqlx::{self, PgPool};
use tracing::{error, info};
use uuid::Uuid;

use crate::startup::AppState;

#[derive(serde::Serialize, Debug, serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// #[tracing::instrument] creates a span at the beginning of the function invocation and automatically attaches all arguments passed to the function to the context of the span
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, appstate)
    fields(
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
// subscriber handler
pub async fn subscribe(
    State(appstate): State<AppState>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    let connection_pool = appstate.connection_pool;

    let request_id = Uuid::new_v4();
    info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id, form.name, form.email
    );

    match insert_subscriber(connection_pool, form, request_id).await {
        Ok(_) => {
            info!("request_id {} - Saved new subscriber details!", request_id);
            StatusCode::OK
        }
        Err(e) => {
            error!("request_id {} - Failed to execute query {}", request_id, e);
            // println!("Failed to Insert into Database due to {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database" ,
    skip(form,connection_pool)
    fields(
        id = %request_id,
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
pub async fn insert_subscriber(
    connection_pool: PgPool,
    form: FormData,
    request_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
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
    .map_err(|e| {
        tracing::error!("Failed to insert the data: {}", e);
        e
    })?;

    Ok(())
}
