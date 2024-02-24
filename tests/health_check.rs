use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application
fn spawn_app() -> String {
    // Create listener to bind a random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Get the random port
    let port = listener.local_addr().unwrap().port();
    // Launch the application
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Handle triggering the server to start similar to what tokio::main does!
    let _ = tokio::spawn(server);
    // Reply back with the address app is running on!
    format!("http://127.0.0.1:{}", port)
}
