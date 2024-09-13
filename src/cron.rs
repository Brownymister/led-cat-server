use crate::config;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use uuid::Uuid;

pub fn new_con_job(schedule_job: crate::config::ConfigSchedule, auth_token: &str, stop_id: &i32) {
    let cron_job = format!(
        r#"{} root {}{}"#,
        schedule_job.cronjob,
        get_curl_by_job_name(&schedule_job, config::Config::new().expect("config error")).unwrap(),
        "\n",
    );
    log::info!("new cron job: {}", cron_job);

    let file_path = "/etc/cron.d/led_cat_cron";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open file");

    // Write the cron job string to the file
    file.write_all(cron_job.as_bytes())
        .expect("Failed to write to file");
}

pub fn delete_cronjob_file() {
    let file_path = "/etc/cron.d/led_cat_cron";
    let _ = std::fs::remove_file(file_path).expect("Failed to delete file");
}

pub fn get_curl_by_job_name(
    schedule_job: &crate::config::ConfigSchedule,
    config: crate::config::Config,
) -> Result<String, ()> {
    let job_name = match schedule_job.job_name.as_str() {
        "show_bvg_info" => {
            let first_str = r#"curl -X GET -d '{"stop_id":"#.to_string()
                + config.bvg_stop_id.to_string().as_str();
            let second_str =
                r#", "speed":50}' -H 'Content-Type: application/json' -H 'Authorization: Bearer "#
                    .to_owned()
                    + &config.automation_auth_token;
            let third_str = r#"' http://localhost:8080/api/show_bvg_info"#;
            let str = first_str + &second_str + third_str;
            return Ok(str);
        }
        "show_weather_info" => {
            let mut day_str = "".to_string();
            if schedule_job.day.is_some() {
                day_str = format!(r#","day": {}"#, schedule_job.day.unwrap_or(0));
            }
            let str = format!(
                r#"curl -X GET -d '{{"lat": {}, "lon": {}{}}}' -H 'Content-Type: application/json' -H 'Authorization: Bearer {}' http://192.168.1.120:8080/api/show_weather_info"#,
                schedule_job.osm_lat.unwrap_or(0.0),
                schedule_job.osm_lon.unwrap_or(0.0),
                day_str,
                &config.automation_auth_token
            );
            return Ok(str.to_string());
        }
        _ => Err(()),
    };
    return job_name;
}
