use dotenv::dotenv;

#[tokio::test]
async fn health_check_works() {
    dotenv().ok();
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8082/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero_to_production::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
