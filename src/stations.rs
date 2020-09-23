use crate::NwsClient;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::common::StateAbbr;


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsStationGeometry {
    #[serde(rename(deserialize = "type"))]
    pub geo_type: String,
    pub coordinates: Vec<f32>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsStationElevation {
    pub value: f32,
    #[serde(rename(deserialize = "unitCode"))]
    pub unit_code: String,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsStationProperties{
    #[serde(rename(deserialize = "@id"))]
    pub id: String,
    #[serde(rename(deserialize = "@type"))]
    pub station_type: String,
    pub elevation: NwsStationElevation,
    #[serde(rename(deserialize = "stationIdentifier"))]
    pub station_identifier: String,
    pub name: String,
    #[serde(rename(deserialize = "timeZone"))]
    pub time_zone: String,
    pub forecast: String,
    pub county: String,
    #[serde(rename(deserialize = "fireWeatherZone"))]
    pub fire_weather_zone: String

}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsStationInfo {
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub station_type: String,
    pub geometry: NwsStationGeometry,
    pub properties: NwsStationProperties
}


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct NwsStationsResponse {
    #[serde(rename(deserialize = "type"))]
    pub stations_response_type: String,
    pub features: Vec<NwsStationInfo>,

}


pub trait NwsStation {
    fn get_all_stations(&self, state_abbr: Option<StateAbbr>, limit: Option<u32>) -> Result<NwsStationsResponse>;
    fn get_station_by_id(&self, station_id: &str) -> Result<NwsStationInfo>;
    //TODO implement /stations/{stationId}/observations
    //TODO implement /stations/{stationId}/observations/latest
    //TODO implement /stations/{stationId}/observations/{time}
}

impl NwsStation for NwsClient {
    //TODO Implement query params
    /// Returns a list of observation stations
    ///
    /// ```
    /// use nws_rs::NwsClient;
    /// use nws_rs::stations::NwsStation;
    /// use nws_rs::common::StateAbbr;
    ///
    /// let client: NwsClient = NwsClient::new(None);
    ///
    /// let response = client.get_all_stations(Some(StateAbbr::IA), Some(25));
    /// ```
    fn get_all_stations(&self, state_abbr: Option<StateAbbr>, limit: Option<u32>) -> Result<NwsStationsResponse> {
        let maybe_state: String = match state_abbr {
            Some(state_abb) => format!("?state={:?}", &state_abb),
            None => String::new(),
        };
        let maybe_limit: String = match limit {
            Some(limit) => format!("&limit={}", &limit),
            None => String::new(),
        };
        let host = format!("{}/stations{}{}", &self.host, &maybe_state, &maybe_limit);
        println!("{}", host);
        let nws_station_response: NwsStationsResponse = self.client.get(&host).send()?.json()?;
        Ok(nws_station_response)
    }

    /// Returns metadata about a given observation station
    ///
    /// ```
    /// use nws_rs::NwsClient;
    /// use nws_rs::stations::NwsStation;
    ///
    /// let client: NwsClient = NwsClient::new(None);
    ///
    /// let response = client.get_station_by_id("COOPZXJI4");
    /// ```
        fn get_station_by_id(&self, station_id: &str) -> Result<NwsStationInfo> {
        let host = format!("{}/stations/{}", &self.host, station_id);
        let nws_station_info: NwsStationInfo = self.client.get(&host).send()?.json()?;
        Ok(nws_station_info)
    }
}