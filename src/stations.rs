use crate::NwsClient;
use serde::{Deserialize, Serialize};
use anyhow::Result;


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsStationsResponse {
    #[serde(rename(deserialize = "@context"))]
    pub context: String,

}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsStationInfo {
    pub term: String,
    pub definition: String,
}