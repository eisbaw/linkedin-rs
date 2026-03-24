use linkedin_api::client::LinkedInClient;

#[test]
fn client_creates_successfully() {
    let _client = LinkedInClient::new();
}

#[test]
fn client_default_works() {
    let _client = LinkedInClient::default();
}

#[test]
fn error_display() {
    let err = linkedin_api::error::Error::Auth("test".to_string());
    assert_eq!(format!("{err}"), "Auth error: test");
}
