use std::net::SocketAddr;

use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

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
        .with_state(appstate)
}

pub async fn run(connection_pool: PgPool, listener: SocketAddr) {
    let appstate = AppState { connection_pool };
    let app = router(appstate);

    // run it with hyper on localhost:3000
    axum::Server::bind(&listener)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
