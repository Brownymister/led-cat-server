use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DwdPollen {
    content: Vec<DwdPollenContent>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DwdPollenContent {
    region_id: i32,
    partregion_id: i32,
    partregion_name: String,
    Pollen: DwdPollenContentPollen,
    region_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DwdPollenContentPollen {
    Roggen: DwdPollenContentPollenPreview,
    Birke: DwdPollenContentPollenPreview,
    Hasel: DwdPollenContentPollenPreview,
    Ambrosia: DwdPollenContentPollenPreview,
    Graeser: DwdPollenContentPollenPreview,
    Beifuss: DwdPollenContentPollenPreview,
    Esche: DwdPollenContentPollenPreview,
    Erle: DwdPollenContentPollenPreview,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DwdPollenContentPollenPreview {
    today: String,
    dayafter_to: String,
    tomorrow: String,
}
