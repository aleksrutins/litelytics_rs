use serde_json::json;
use reqwest::Error;
mod types;
#[macro_use] mod macros;
mod api_helpers;
use api_helpers::ApiHelpers;
use types::*;

// API client
pub struct Client {
    token: String,
    instance_url: String,
    uid: u32
}
impl ApiHelpers for Client {
    fn instance_url(&self) -> String {
        self.instance_url.to_owned()
    }
    fn token(&self) -> String {
        self.token.to_owned()
    }
}
// Client implementation
impl Client {
    // Log in; constructor
    pub async fn log_in(data: LogInPayload) -> Option<Self> {
        let client = reqwest::Client::new();
        let resp: LogInResult = serde_json::from_str(
            &response_text!(request client.post(format!("{}/api/user/{}/sign-in", data.instance_url, data.email)).body(json!({ "password": data.password }).to_string()))
        ).ok()?;

        Some(Client {
            token: resp.token.clone(),
            instance_url: data.instance_url.clone(),
            uid: resp.userId
        })
    }

    // Set the token
    pub fn set_token(&mut self, token: String, user_id: u32) {
        self.token = token.clone();
        self.uid = user_id;
    }
    // Set the instance URL
    pub fn set_instance(&mut self, instance_url: String) {
        self.instance_url = instance_url.clone();
    }

    // Get the current user's sites
    pub async fn get_sites(&self) -> Option<Vec<u32>> {
        let client = reqwest::Client::new();
        let resp: SiteList = serde_json::from_str(
            &response_text!(request self.api_get(&client, "/api/site/list"))
        ).ok()?;
        Some(resp.sites)
    }

    // Get info for a particular site
    pub async fn get_site_info(&self, site: u32) -> Option<SiteInfo> {
        let client = hclient!();
        Some(serde_json::from_str::<InfoWrapper>(
            &response_text!(
                request self.api_get(&client, &format!("/api/site/{}/info", site))
            )
        ).ok()?.info)
    }

    // Get data for a particular site
    pub async fn get_site_data(&self, site: u32) -> Option<Vec<Visit>> {
        let client = hclient!();
        Some(serde_json::from_str::<DataWrapper>(
            &response_text!(
                request self.api_get(&client, &format!("/api/site/{}/data", site))
            )
        ).ok()?.data)
    }

    // Add a user to a site
    pub async fn add_user_to_site(&self, site: u32, user: u32) -> Result<(), Error> {
        let client = hclient!();
        send_request!(self.api_post(&client, &format!("/api/site/{}/user/{}/add", site, user), json!({})))?;
        Ok(())
    }
}
#[cfg(test)]
mod tests;