use crate::NwsClient;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsGlossaryResponse {
    pub glossary: Vec<NwsGlossaryInfo>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsGlossaryInfo {
    pub term: String,
    pub definition: String,
}

pub trait NwsGlossary {
    fn get_glossary_terms(&self) -> Result<NwsGlossaryResponse>;
}

impl NwsGlossary for NwsClient {
    fn get_glossary_terms(&self) -> Result<NwsGlossaryResponse> {
        let host = format!("{}/glossary", &self.host);
        let nws_alert: NwsGlossaryResponse = self.client.get(&host).send()?.json()?;
        Ok(nws_alert)
    }
}
