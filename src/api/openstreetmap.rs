use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct OpenStreetMapAddressdetails {
    pub place_id: i64,
    pub licence: String,
    pub osm_type: String,
    pub osm_id: i64,
    pub lat: String,
    pub lon: String,
    pub class: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub place_rank: i64,
    pub importance: f64,
    pub addresstype: String,
    pub name: String,
    pub display_name: String,
    pub address: OpenStreetMapAddressdetailsAddress,
    pub boundingbox: [String; 4],
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct OpenStreetMapAddressdetailsAddress {
    pub house_number: Option<String>,
    pub road: Option<String>,
    pub suburb: Option<String>,
    pub borough: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "ISO3166-2-lvl4")]
    pub ISO3166_2_lvl4: Option<String>,
    pub postcode: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub leisure: Option<String>,
    pub hamlet: Option<String>,
    pub town: Option<String>,
    pub municipality: Option<String>,
    pub state: Option<String>,
    pub village: Option<String>,
}
