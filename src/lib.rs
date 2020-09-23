
use reqwest::blocking::Client;
use reqwest::header;
use http::header::ACCEPT;
use url::Url;

pub mod alerts;
pub mod common;
pub mod glossary;
pub mod stations;

#[derive(Debug)]
pub struct NwsClient {
    host: Url,
    //reqwest Client.
    client: Client
}

impl NwsClient {
    #[allow(dead_code)]
    pub fn new(user_agent: Option<String>) -> NwsClient {
        let mut headers = header::HeaderMap::new();
        headers.insert(ACCEPT, "application/geo+json".parse().unwrap());

        let base_url = Url::parse("https://api.weather.gov").unwrap();

        let default_user_agent = match user_agent {
            Some(u) => u,
            None => String::from("nws-rust-implementation"),
        };
        let client: Client = reqwest::blocking::Client::builder()
            .user_agent(&default_user_agent)
            .default_headers(headers)
            .build().unwrap();
        NwsClient{ host: base_url, client }
    }
}




