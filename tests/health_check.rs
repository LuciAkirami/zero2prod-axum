// // Connection is needed to import PgConnection::connect
// use sqlx::{Connection, Executor, PgConnection, PgPool, Pool};
// use uuid::Uuid;
// use zero2prod_axum::configuration::parse_configuration;
// use zero2prod_axum::startup::run;

// #[tokio::test]
// async fn test_simple_base() -> httpc_test::Result<()> {
//     // Create a new httpc test client with a base URL (will be prefixed for all calls)
//     let hc = httpc_test::new_client("http://localhost:3000")?;

//     //// do_get, do_post, do_put, do_patch, do_delete return a httpc_test::Response

//     // Simple do_get
//     let res = hc.do_get("/health_check").await?; // httpc_test::Response
//     let status = res.status();

//     let body = res.header("content-length").unwrap();

//     assert_eq!(status.as_u16(), 200);
//     assert_eq!("0", body);

//     Ok(())
// }

// // #[tokio::test]
// // async fn subscribe_returns_200_for_valid_form() {
// //     // let configuration =
// //     //     configuration::parse_configuration().expect("Failed to read configuration.");

// //     // let connection_string = configuration.database.get_connection_string();

// //     // // The `Connection` trait MUST be in scope for us to invoke
// //     // // `PgConnection::connect` - it is not an inherent method of the struct!

// //     // let mut connection = PgConnection::connect(&connection_string)
// //     //     .await
// //     //     .expect("Falied to connect to Postgres.");

// //     let connection_pool = database_configuration().await;

// //     let client = reqwest::Client::new();

// //     let body = "name=lucifer&email=lucifer%40hell.com";

// //     let response = client
// //         .post("http://localhost:3000/subscriptions")
// //         .header("Content-Type", "application/x-www-form-urlencoded")
// //         .body(body)
// //         .send()
// //         .await
// //         .expect("Failed to execute request.");

// //     let saved_info = sqlx::query!("SELECT email, name FROM subscriptions",)
// //         .fetch_one(&connection_pool)
// //         .await
// //         .expect("Failed to fetch saved subscription.");

// //     assert_eq!(saved_info.email, "lucifer@hell.com");
// //     assert_eq!(saved_info.name, "lucifer");

// //     assert_eq!(response.status().as_u16(), 200);
// // }

// async fn spawn_app() {
//     let configuration = parse_configuration().expect("Failed to read configuration.");
//     println!("{configuration:?}");
//     let connection_string = configuration.database.get_connection_string();
//     println!("{connection_string:?}");

//     // creates a single connection
//     // let connection = sqlx::PgConnection::connect(&connection_string)
//     //     .await
//     //     .expect("Falied to connect to Postgres.");

//     // creates a pool of connections, so that even if one connection is slow, sqlx can use the
//     // other connection to perform operations
//     let connection_pool = sqlx::PgPool::connect(&connection_string)
//         .await
//         .expect("Falied to connect to Postgres.");

//     run(connection_pool).await
// }

// #[sqlx::test]
// async fn subscribe_returns_200_for_valid_form(pool: PgPool) -> sqlx::Result<()> {
//     // let c
//     Ok(())
// }

// #[tokio::test]
// async fn subscibe_returns_400_for_invalid_form() {
//     let client = reqwest::Client::new();

//     let body = "email=Lucifer";

//     let response = client
//         .post("http://localhost:3000/subscriptions")
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     // Status Code 422(Unprocessable Entity) is returned for invalid form data
//     assert_eq!(response.status().as_u16(), 422);
// }

// // async fn database_configuration() -> PgPool {
// //     let mut configuration = parse_configuration().expect("Failed to Parse Configuration");

// //     configuration.database.database_name = Uuid::new_v4().to_string();
// //     println!("{configuration:?}");

// //     let mut connection =
// //         PgConnection::connect(&configuration.database.get_connection_string_without_db())
// //             .await
// //             .expect("Failed to connect");

// //     let dbname = configuration.database.get_connection_string();
// //     println!("{dbname}");
// //     connection
// //         .execute(
// //             format!(
// //                 r#"CREATE DATABASE {};"#,
// //                 configuration.database.database_name
// //             )
// //             .as_str(),
// //         )
// //         .await
// //         .expect("Failed to create database");

// //     let connection_pool = PgPool::connect(&configuration.database.get_connection_string())
// //         .await
// //         .expect("Failed to connect");

// //     sqlx::migrate!("./migrations")
// //         .run(&connection_pool)
// //         .await
// //         .expect("Faield to run migrations");
// //     connection_pool
// // }
use sqlx::PgPool;
use tokio::net::TcpListener;

use zero2prod_axum::startup;

async fn setup_app(pool: PgPool) -> u16 {
    let address = format!("127.0.0.1:0");
    let listener = TcpListener::bind(address)
        .await
        .unwrap()
        .local_addr()
        .unwrap();
    let port = listener.port();

    let app = startup::run(pool, listener);
    tokio::spawn(app);

    port
}

#[sqlx::test(migrations = "./migrations")]
async fn subscibe_returns_ok_and_saved_in_db(pool: PgPool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;
    let port = setup_app(pool).await;
    let client = reqwest::Client::new();
    let body = "name=lucifer&email=lucifer%40hell.com";

    let response = client
        .post(format!("http://localhost:{}/subscriptions", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let saved_info = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut *conn)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved_info.email, "lucifer@hell.com",);
    assert_eq!(saved_info.name, "lucifer");
    assert_eq!(response.status().as_u16(), 200);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn subscibe_returns_error(pool: PgPool) -> sqlx::Result<()> {
    let port = setup_app(pool).await;
    let client = reqwest::Client::new();

    let body = "name=akirami&email=lucifer%40hell.com";

    let _response = client
        .post(format!("http://localhost:{}/subscriptions", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // giving in the same email address to check for duplicates

    let body = "name=Inamii&email=lucifer%40hell.com";

    let response2 = client
        .post(format!("http://localhost:{}/subscriptions", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response2.status().as_u16(), 500);

    Ok(())
}
