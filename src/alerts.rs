use crate::NwsClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsAlertProperties {
    pub id: String,
    #[serde(rename(deserialize = "areaDesc"))]
    pub area_desc: String,
}


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsAlert {
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub nws_alert_type: String,
    pub properties: NwsAlertProperties,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsAlertsResponse {
    #[serde(rename(deserialize = "@context"))]
    pub context: Value,
    pub features: Vec<NwsAlert>,
    pub title: String,
    pub updated: String,
}


pub trait NwsAlerts {
    fn get_all_alerts(&self) -> Result<NwsAlertsResponse>;
}

impl NwsAlerts for NwsClient {
    fn get_all_alerts(&self) -> Result<NwsAlertsResponse> {
        let host = format!("{}/alerts?active=true", &self.host);
        let nws_alert: NwsAlertsResponse = self.client.get(&host).send()?.json()?;
        Ok(nws_alert)
    }
}
