use screensage::OllamaClient;
use mockito::Server;

#[tokio::test]
async fn test_client_initialization() {
    // Test with valid URL
    let client = OllamaClient::new("http://localhost:11434").unwrap();
    assert_eq!(client.api_url(), "http://localhost:11434");

    // Test with URL that has trailing slash
    let client = OllamaClient::new("http://localhost:11434/").unwrap();
    assert_eq!(client.api_url(), "http://localhost:11434");

    // Test with invalid URL
    let result = OllamaClient::new("localhost:11434");
    assert!(result.is_err());
}

// Note: The following tests would require mockito to be set up properly
// These are commented out until the proper mocking is implemented

/*
#[tokio::test]
async fn test_list_models_mock() {
    // Create a mock server
    let mut server = Server::new();
    
    // Setup mock response
    let mock = server.mock("GET", "/api/tags")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"models":[{"name":"llama2","size":3791730298,"modified_at":"2023-10-15T14:32:10Z"}]}"#)
        .create();
    
    // Create client with mock server URL
    let client = OllamaClient::new(&server.url()).unwrap();
    
    // Test the API call
    let response = client.list_models().await.unwrap();
    
    // Verify the mock was called
    mock.assert();
    
    // Verify the response
    assert_eq!(response.models.len(), 1);
    assert_eq!(response.models[0].name, "llama2");
}
*/
