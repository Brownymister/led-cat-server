use serde::{Deserialize, Serialize};

// pub struct BvgDeparture {
//     pub departures: Vec<BvgTimeTable>,
// }
//
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub type BvgDeparture = Vec<BvgTimeTable>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BvgTimeTable {
    pub stop: BvgTimeTableStop,
    pub when: Option<String>,
    pub planned_when: Option<String>,
    pub delay: Option<i32>,
    pub platform: Option<String>,
    pub planned_platform: Option<String>,
    pub direction: Option<String>,
    pub line: Option<BvgTimeTableLine>,
    pub occupancy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BvgTimeTableStop {
    pub name: String,
    pub location: BvgTimeTableStopLocation,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BvgTimeTableStopLocation {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BvgTimeTableLine {
    pub id: String,
    pub name: String,
    pub products: Option<String>,
    pub mode: Option<String>,
    #[serde(rename = "productName")]
    pub product_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BvgTimeTableRemark {
    pub code: String,
    pub text: String,
    #[serde(rename = "type")]
    pub type_: String,
}
