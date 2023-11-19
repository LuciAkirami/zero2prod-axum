use zero2prod_axum::{configuration::parse_configuration, startup::run};

#[tokio::main]
async fn main() {
    let configuration = parse_configuration().expect("Failed to read configuration.");
    println!("{configuration:?}");
    let connection_string = configuration.database.get_connection_string();
    println!("{connection_string:?}");

    // creates a single connection
    // let connection = sqlx::PgConnection::connect(&connection_string)
    //     .await
    //     .expect("Falied to connect to Postgres.");

    // creates a pool of connections, so that even if one connection is slow, sqlx can use the
    // other connection to perform operations
    let connection_pool = sqlx::PgPool::connect(&connection_string)
        .await
        .expect("Falied to connect to Postgres.");

    run(connection_pool).await;
}
