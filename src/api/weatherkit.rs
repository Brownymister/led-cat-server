use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct WeatherForcast {
    // pub forecastDaily: Option<WeatherDailyForcast>,
    // pub forecastHourly: Option<WeatherHourlyForcast>,
    pub forecastDaily: WeatherDailyForcast,
    pub forecastHourly: WeatherHourlyForcast,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct WeatherDailyForcast {
    pub metadata: WeatherDailyForcastMetadata,
    pub days: Vec<WeatherDailyForcastDays>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct WeatherHourlyForcast {
    pub metadata: WeatherDailyForcastMetadata,
    pub hours: Vec<WeatherHourlyForcastHours>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct WeatherDailyForcastMetadata {
    pub attributionURL: String,
    pub expireTime: String,
    pub latitude: f32,
    pub longitude: f32,
    pub readTime: String,
    pub reportedTime: String,
    pub units: String,
    pub version: f32,
    pub sourceType: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct WeatherDailyForcastDays {
    pub forecastStart: String,
    pub forecastEnd: String,
    pub conditionCode: String,
    pub maxUvIndex: f32,
    pub moonPhase: String,
    pub moonrise: Option<String>,
    pub moonset: Option<String>,
    pub precipitationAmount: f32,
    pub precipitationChance: f32,
    pub precipitationType: String,
    pub snowfallAmount: f32,
    pub solarMidnight: String,
    pub solarNoon: String,
    pub sunrise: String,
    pub sunriseCivil: String,
    pub sunriseNautical: String,
    pub sunriseAstronomical: Option<String>,
    pub sunset: String,
    pub sunsetCivil: String,
    pub sunsetNautical: String,
    pub sunsetAstronomical: Option<String>,
    pub temperatureMax: f32,
    pub temperatureMin: f32,
    pub windGustSpeedMax: f32,
    pub windSpeedAvg: f32,
    pub windSpeedMax: f32,
    pub daytimeForecast: DayliForcast,
    pub overnightForecast: DayliForcast,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct WeatherHourlyForcastHours {
    pub forecastStart: String,
    pub cloudCover: f32,
    pub conditionCode: String,
    pub daylight: bool,
    pub humidity: f32,
    pub precipitationAmount: f32,
    pub precipitationIntensity: f32,
    pub precipitationChance: f32,
    pub precipitationType: String,
    pub pressure: f32,
    pub pressureTrend: String,
    pub snowfallIntensity: f32,
    pub snowfallAmount: f32,
    pub temperature: f32,
    pub temperatureApparent: f32,
    pub temperatureDewPoint: f32,
    pub uvIndex: f32,
    pub visibility: f32,
    pub windDirection: f32,
    pub windGust: f32,
    pub windSpeed: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct DayliForcast {
    pub forecastStart: String,
    pub forecastEnd: String,
    pub cloudCover: f32,
    pub conditionCode: String,
    pub humidity: f32,
    pub precipitationAmount: f32,
    pub precipitationChance: f32,
    pub precipitationType: String,
    pub snowfallAmount: f32,
    pub temperatureMax: f32,
    pub temperatureMin: f32,
    pub windDirection: f32,
    pub windGustSpeedMax: f32,
    pub windSpeed: f32,
    pub windSpeedMax: f32,
}
