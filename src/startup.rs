use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};

use crate::routes::{health_check, subscribe};

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

pub fn router() -> Router {
    // build our application with a single route
    Router::new()
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run() {
    let app = router();
    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
