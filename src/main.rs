// use sqlx::database;
// use zero2prod_axum::run;
use zero2prod_axum::{configuration::parse_configuration, startup::run};

#[tokio::main]
async fn main() {
    let configuration = parse_configuration().expect("Failed to read configuration.");
    println!("{configuration:?}");
    let connection_string = configuration.database.get_connection_string();
    println!("{connection_string:?}");
    run().await;
}
