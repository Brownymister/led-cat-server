use std::io::Write;
use std::io;

use reqwest::header::USER_AGENT;

pub async fn get_bvg_info(stop_id: u64) -> String {
    let response = reqwest::get(get_bvg_stop_url(stop_id))//.await;
        .await.expect("to work")
        .text()
        .await.expect("to work");


    // match response {
    //     Ok(success) => {
    //         // Handle successful response
    //         println!("Response: {:?}", success.text().await.unwrap());
    //     },
    //     Err(error) => {
    //         if error.is_timeout() {
    //             // Handle timeout error
    //             writeln!(io::stderr(), "Request timed out").unwrap();
    //         } else if error.is_connect() {
    //             // Handle connection error
    //             writeln!(io::stderr(), "Network connection error").unwrap();
    //         } else {
    //             // Handle other errors
    //             writeln!(io::stderr(), "Error: {}", error).unwrap();
    //         }
    //     },
    // }
    return response;
}


fn get_bvg_stop_url(stop_id: u64) -> String {
    return format!("https://v6.db.transport.rest/stops/{}/departures?results=10", stop_id);
}

pub async fn get_weather_info(lat: f32, lon: f32) -> String {
    let weatherkit_api_token = std::env::var("WEATHERKIT_API_TOKEN").expect("WEATHERKIT_API_TOKEN must be set.");

    let client = reqwest::Client::new();
    let body = client.get(get_weather_url(lat, lon))
        .header("Authorization", format!("Bearer {}", weatherkit_api_token))
        .send()
        .await.expect("to work")
        .text()
        .await.expect("to work");
    println!("{}", body);
    return body;
}

fn get_weather_url(lat: f32, lon: f32) -> String {
    return format!("https://weatherkit.apple.com/api/v1/weather/de_DE/{}/{}?country=DE&timezone=europe%2FBerlin&dataSets=forecastHourly&dataSets=forecastDaily", lat, lon);
}

pub async fn get_osm_info(lat: f32, lon: f32) -> String {
    let osm_user_agent = std::env::var("OSM_USER_AGENT").expect("OSM_USER_AGENT must be set.");

    let client = reqwest::Client::new();
    let body = client.get(get_osm_url(lat, lon))
        .header(USER_AGENT, osm_user_agent)
        .send()
        .await.expect("to work")
        .text()
        .await.expect("to work");
    return body;
}

fn get_osm_url(lat: f32, lon: f32) -> String {
    return format!("https://nominatim.openstreetmap.org/reverse?format=json&lat={}&lon={}&zoom=18&addressdetails=1", lat, lon);
}

pub async fn get_autobahn_info(road_id: String) -> String {
    let client = reqwest::Client::new();
    let body = client.get(get_autobahn_url(road_id))
        .send()
        .await.expect("to work")
        .text()
        .await.expect("to work");
    return body;
}

fn get_autobahn_url(road_id: String) -> String {
    return format!("https://verkehr.autobahn.de/o/autobahn/{}/services/warning", road_id);
}

pub async fn get_radio_song_info(station_name: String) -> String {
    let client = reqwest::Client::new();
    let body = client.get(get_radio_song_url(station_name))
        .send()
        .await.expect("to work")
        .text()
        .await.expect("to work");
    return body;
}

fn get_radio_song_url(station_name: String) -> String {
    return format!("https://myonlineradio.de/{}/playlist", station_name);
}

pub async fn get_pollen_data() -> String {
    let client = reqwest::Client::new();
    let body = client.get("https://opendata.dwd.de/climate_environment/health/alerts/s31fg.json")
        .send()
        .await.expect("to work")
        .text()
        .await.expect("to work");
    return body;
}

pub async fn get_football_data() -> String {
    let client = reqwest::Client::new();
    let body = client.get("https://api.openligadb.de/getmatchdata/em/2024/7")
        .send()
        .await.expect("to work")
        .text()
        .await.expect("to work");
    return body;
}
