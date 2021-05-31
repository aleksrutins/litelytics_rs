use serde_json::json;
use serde::{Deserialize};
pub struct LogInPayload {
    instance_url: String,
    email: String,
    password: String
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct LogInResult {
    token: String,
    userId: u32
}
#[derive(Deserialize)]
struct SiteList {
    sites: Vec<u32>
}
#[derive(Deserialize)]
struct InfoWrapper {
    info: SiteInfo
}
#[derive(Deserialize)]
pub struct SiteInfo {
    pub id: u32,
    pub domain: String
}
#[derive(Deserialize)]
struct DataWrapper {
    data: Vec<Visit>
}
#[derive(Deserialize)]
pub struct Visit {
    pub id: u32,
    pub site: u32,
    pub useragent: String,
    pub path: String,
    pub referer: String,
    pub timestamp: String,
    pub ip: String
}
pub struct Client {
    token: String,
    instance_url: String,
    uid: u32
}
macro_rules! send_request {
    ($req:expr) => {
        $req
        .send()
        .await
        .expect("Failed to send request")
    };
}
macro_rules! response_text {
    ($res:expr) => {
        $res
        .text()
        .await
        .expect("Failed to get response text")
    };
    (request $req:expr) => {
        response_text!(send_request!($req))
    }
}
macro_rules! hclient {
    () => {
        reqwest::Client::new()
    };
}
impl Client {
    fn api_get(&self, client: &reqwest::Client, path: &str) -> reqwest::RequestBuilder {
        client
        .get(format!("{}{}", self.instance_url, path))
        .header("Authorization", format!("Bearer {}", self.token))
    }
    fn api_post(&self, client: &reqwest::Client, path: &str, body: serde_json::Value) -> reqwest::RequestBuilder {
        client
        .post(format!("{}{}", self.instance_url, path))
        .body(body.to_string())
        .header("Authorization", format!("Bearer {}", self.token))
    }
    pub async fn log_in(data: LogInPayload) -> Self {
        let client = reqwest::Client::new();
        let resp: LogInResult = serde_json::from_str(
            &response_text!(request client.post(format!("{}/api/user/{}/sign-in", data.instance_url, data.email)).body(json!({ "password": data.password }).to_string()))
        ).expect("Invalid response");

        Client {
            token: resp.token.clone(),
            instance_url: data.instance_url.clone(),
            uid: resp.userId
        }
    }
    pub fn set_token(&mut self, token: String, user_id: u32) {
        self.token = token.clone();
        self.uid = user_id;
    }
    pub fn set_instance(&mut self, instance_url: String) {
        self.instance_url = instance_url.clone();
    }
    pub async fn get_sites(&self) -> Vec<u32> {
        let client = reqwest::Client::new();
        let resp: SiteList = serde_json::from_str(
            &response_text!(request self.api_get(&client, "/api/site/list"))
        ).expect("Failed to deserialize");
        resp.sites
    }
    pub async fn get_site_info(&self, site: u32) -> SiteInfo {
        let client = hclient!();
        serde_json::from_str::<InfoWrapper>(
            &response_text!(
                request self.api_get(&client, &format!("/api/site/{}/info", site))
            )
        ).expect("Failed to deserialize").info
    }
    pub async fn get_site_data(&self, site: u32) -> Vec<Visit> {
        let client = hclient!();
        serde_json::from_str::<DataWrapper>(
            &response_text!(
                request self.api_get(&client, &format!("/api/site/{}/data", site))
            )
        ).expect("Failed to deserialize").data
    }
    pub async fn add_user_to_site(&self, site: u32, user: u32) {
        let client = hclient!();
        send_request!(self.api_post(&client, &format!("/api/site/{}/user/{}/add", site, user), json!({})));
    }
}