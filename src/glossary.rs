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
    /// Returns a list of glossary terms
    ///
    /// ```
    /// use nws_rs::NwsClient;
    /// use nws_rs::glossary::NwsGlossary;
    ///
    /// let client: NwsClient = NwsClient::new(None);
    ///
    /// let response = client.get_glossary_terms();
    /// ```
    fn get_glossary_terms(&self) -> Result<NwsGlossaryResponse> {
        let url = self.host.join("glossary")?;
        let nws_alert: NwsGlossaryResponse = self.client.get(url).send()?.json()?;
        Ok(nws_alert)
    }
}