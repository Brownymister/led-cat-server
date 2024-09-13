use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct AutobahnWarningsBody {
    pub warning: Vec<AutobahnWarning>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct AutobahnWarning {
    pub identifier: String,
    pub icon: String,
    // maybe string
    pub isBlocked: String,
    pub future: bool,
    pub extent: String,
    pub point: String,
    pub startLcPosition: String,
    pub display_type: String,
    pub subtitle: String,
    pub title: String,
    pub startTimestamp: String,
    pub coordinate: AutobahnCoordinates,
    pub description: Vec<String>,
    pub routeRecommendation: Vec<String>,
    pub footer: Vec<String>,
    pub lorryParkingFeatureIcons: Vec<String>,
    pub abnormalTrafficType: Option<String>,
    pub delayTimeValue: Option<String>,
    pub averageSpeed: Option<String>,
    pub geometry: Option<AutobahnGeometry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct AutobahnCoordinates {
    pub lat: f64,
    pub long: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct AutobahnGeometry {
    #[serde(rename = "type")]
    pub _type: String,
    pub coordinates: Vec<[f64; 2]>,
}

pub fn match_trafic_type(abnormal_traffic_type: String) -> String {
    return match abnormal_traffic_type.as_str() {
        "STATIONARY_TRAFFIC" => "Mega Stau".to_string(),
        // "QUEUING_TRAFFIC" => "Stau".to_string(),
        "SLOW_TRAFFIC" => "Kleiner Stau".to_string(),
        _ => "Stau".to_string(),
    };
}
