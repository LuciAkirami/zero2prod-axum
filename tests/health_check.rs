// use anyhow::Result;
// use serde_json::{json, Value};

// Connection is need to import PgConnection::connect
// use sqlx::{Connection, PgConnection};
// use zero2prod_axum::configuration;

#[tokio::test]
async fn test_simple_base() -> httpc_test::Result<()> {
    // Create a new httpc test client with a base URL (will be prefixed for all calls)
    let hc = httpc_test::new_client("http://localhost:3000")?;

    //// do_get, do_post, do_put, do_patch, do_delete return a httpc_test::Response

    // Simple do_get
    let res = hc.do_get("/health_check").await?; // httpc_test::Response
    let status = res.status();

    let body = res.header("content-length").unwrap();

    assert_eq!(status.as_u16(), 200);
    assert_eq!("0", body);

    Ok(())
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form() {
    // let configuration =
    //     configuration::parse_configuration().expect("Failed to read configuration.");

    // let connection_string = configuration.database.get_connection_string();

    // let mut connection = PgConnection::connect(&connection_string)
    //     .await
    //     .expect("Falied to connect to Postgres.");

    let client = reqwest::Client::new();

    let body = "name=lucifer&email=lucifer%40hell.com";

    let response = client
        .post("http://localhost:3000/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // let saved_info = sqlx::query!("SELECT email, name FROM subscriptions",)
    //     .fetch_one(&mut connection)
    //     .await
    //     .expect("Failed to fetch saved subscription.");

    // assert_eq!(saved_info.email, "lucifer@hell.com");
    // assert_eq!(saved_info.name, "lucifer");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscibe_returns_400_for_invalid_form() {
    let client = reqwest::Client::new();

    let body = "email=Lucifer";

    let response = client
        .post("http://localhost:3000/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Status Code 422(Unprocessable Entity) is returned for invalid form data
    assert_eq!(response.status().as_u16(), 422);
}
