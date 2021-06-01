use serde::{Deserialize};
// Data to pass to Client::log_in
pub struct LogInPayload {
    pub instance_url: String,
    pub email: String,
    pub password: String
}

// Result of login API call
#[derive(Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub(crate) struct LogInResult {
    pub success: bool,
    pub token: String,
    pub userId: u32
}

// List of site IDs
#[derive(Deserialize)]
pub struct SiteList {
    pub sites: Vec<u32>
}

// Response from site info API
#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct InfoWrapper {
    pub success: bool,
    pub info: SiteInfo
}

// Site information
#[derive(Deserialize)]
pub struct SiteInfo {
    pub id: u32,
    pub domain: String
}

// Response from data API call
#[derive(Deserialize)]
pub(crate) struct DataWrapper {
    pub data: Vec<Visit>
}

// One visit to a site
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