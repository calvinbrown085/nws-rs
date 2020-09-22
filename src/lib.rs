
use reqwest::blocking::Client;

mod alerts;
mod glossary;

#[derive(Debug)]
pub struct NwsClient {
    host: String,
    //reqwest Client.
    client: Client
}

impl NwsClient {
    #[allow(dead_code)]
    fn new(user_agent: Option<String>) -> NwsClient {
        let default_user_agent = match user_agent {
            Some(u) => u,
            None => String::from("nws-rust-implementation"),
        };
        let client: Client = reqwest::blocking::Client::builder()
            .user_agent(&default_user_agent)
            .build().unwrap();
        NwsClient{ host: String::from("https://api.weather.gov"), client }
    }
}




