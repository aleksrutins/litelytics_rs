pub trait ApiHelpers {
    fn instance_url(&self) -> String;
    fn token(&self) -> String;
    // Authenticated GET request
    fn api_get(&self, client: &reqwest::Client, path: &str) -> reqwest::RequestBuilder {
        client
        .get(format!("{}{}", self.instance_url(), path))
        .header("Authorization", format!("Bearer {}", self.token()))
    }

    // Authenticated POST request
    fn api_post(&self, client: &reqwest::Client, path: &str, body: serde_json::Value) -> reqwest::RequestBuilder {
        client
        .post(format!("{}{}", self.instance_url(), path))
        .body(body.to_string())
        .header("Authorization", format!("Bearer {}", self.token()))
    }
}