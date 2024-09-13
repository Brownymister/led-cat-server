use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub bvg_stop_id: i32,
    pub automation_auth_token: String,
    pub schedules: Option<Vec<ConfigSchedule>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigScheduleLedFuncData {
    pub info_message: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigSchedule {
    pub description: String,
    pub job_name: String,
    pub cronjob: String,
    pub job_led_func_data: ConfigScheduleLedFuncData,
    pub osm_lat: Option<f32>,
    pub osm_lon: Option<f32>,
    pub day: Option<i32>,
}

impl Config {
    pub fn new() -> Result<Config, std::io::Error> {
        let contents = fs::read_to_string("/home/pi/.config/ledcat/config.json")?;
        let c: Config = serde_json::from_str(contents.as_str())?;
        return Ok(c);
    }

    pub fn update(self) -> Result<Config, std::io::Error> {
        log::info!("updating config to : {:?}", self);
        let contents = serde_json::to_string(&self)?;
        fs::write("/home/pi/.config/ledcat/config.json", contents)?;
        return Ok(self);
    }
}
