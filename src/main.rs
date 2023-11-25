// use env_logger::Env;
use tokio::net::TcpListener;

use zero2prod_axum::{configuration::parse_configuration, startup::run, telemetry};

#[tokio::main]
async fn main() {
    let configuration = parse_configuration().expect("Failed to read configuration.");
    println!("{configuration:?}");
    let connection_string = configuration.database.get_connection_string();
    println!("{connection_string:?}");

    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = telemetry::get_subscriber("zero2prod_axum".into(), "info".into());
    telemetry::init_subcriber(subscriber);

    // creates a single connection
    // let connection = sqlx::PgConnection::connect(&connection_string)
    //     .await
    //     .expect("Falied to connect to Postgres.");

    // creates a pool of connections, so that even if one connection is slow, sqlx can use the
    // other connection to perform operations
    let connection_pool = sqlx::PgPool::connect(&connection_string)
        .await
        .expect("Falied to connect to Postgres.");

    let address = "127.0.0.1:0";
    let listener = TcpListener::bind(address)
        .await
        .unwrap()
        .local_addr()
        .unwrap();
    // info!("Listening to {}", listener);
    run(connection_pool, listener).await;
}
