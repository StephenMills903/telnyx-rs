use telnyx_rs::TelnyxClient;
use wiremock::MockServer;

pub struct TestContext {
    pub server: MockServer,
    pub client: TelnyxClient,
}

pub async fn setup() -> TestContext {
    let server = MockServer::start().await;
    let client = TelnyxClient::builder()
        .api_key("test-api-key")
        .base_url(server.uri())
        .build()
        .expect("Failed to build test client");

    TestContext { server, client }
}
