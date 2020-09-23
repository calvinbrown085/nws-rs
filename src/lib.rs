
use reqwest::blocking::Client;
use reqwest::header;
use http::header::ACCEPT;

pub mod alerts;
pub mod common;
pub mod glossary;
pub mod stations;

#[derive(Debug)]
pub struct NwsClient {
    host: String,
    //reqwest Client.
    client: Client
}

impl NwsClient {
    #[allow(dead_code)]
    pub fn new(user_agent: Option<String>) -> NwsClient {
        let mut headers = header::HeaderMap::new();
        headers.insert(ACCEPT, "application/geo+json".parse().unwrap());

        let default_user_agent = match user_agent {
            Some(u) => u,
            None => String::from("nws-rust-implementation"),
        };
        let client: Client = reqwest::blocking::Client::builder()
            .user_agent(&default_user_agent)
            .default_headers(headers)
            .build().unwrap();
        NwsClient{ host: String::from("https://api.weather.gov"), client }
    }
}




