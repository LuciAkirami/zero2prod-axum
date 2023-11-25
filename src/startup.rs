use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::info;
use tracing::instrument;
use tracing::Level;

use crate::routes::{health_check, subscribe};

#[derive(Clone)]
pub struct AppState {
    pub connection_pool: PgPool,
}

pub fn router(appstate: AppState) -> Router {
    // build our application with a single route
    Router::new()
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            TraceLayer::new_for_http().on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Millis),
            ),
        )
        .with_state(appstate)
}

pub async fn run(connection_pool: PgPool, listener: SocketAddr) {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::INFO)
    //     // .json()
    //     .init();

    let appstate = AppState { connection_pool };
    let app = router(appstate);

    info!("Listening to {}", listener);

    // run it with hyper on localhost:3000
    axum::Server::bind(&listener)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[instrument]
async fn greet(Path(name): Path<String>) -> String {
    info!("Handling route");
    format!("Hello, {}!", name)
}
