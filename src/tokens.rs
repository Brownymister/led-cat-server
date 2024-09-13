use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{metadata, File, OpenOptions};
use std::io::{self, prelude::*, Read, Write};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Token {
    token: String,
    expiration_date: String,
    purpuse: Option<String>,
}

impl Token {
    pub fn new(mut expiration_date: Option<DateTime<Utc>>, purpuse: Option<String>) -> Token {
        let mut future_timestamp_string;
        if expiration_date.is_none() {
            let now = Utc::now();
            let one_year_duration = Duration::days(365);
            let future_timestamp = now + one_year_duration;

            future_timestamp_string = future_timestamp.format("%Y-%m-%dT%H:%M:%S.%3fZ");
        } else {
            future_timestamp_string = expiration_date.unwrap().format("%Y-%m-%dT%H:%M:%S.%3fZ");
        }
        let token_str = Uuid::new_v4().to_string();
        log::info!("creating token - {}", token_str);
        return Token {
            token: token_str,
            expiration_date: future_timestamp_string.to_string(),
            purpuse,
        };
    }

    pub fn save(self) -> Result<String, std::io::Error> {
        let file_path = "/home/pi/.config/ledcat/token.txt";

        log::info!("i got here");
        let mut contents = String::new();

        if metadata(file_path).is_ok() {
            let mut file = File::open(file_path)?;
            file.read_to_string(&mut contents)?;
        }
        let mut file = File::create(file_path).expect("to work");

        if contents == "" {
            contents = "[]".to_string();
        }
        let mut objects: Vec<Token> = serde_json::from_str(&contents)?;

        objects.push(self.clone());

        let updated_contents = serde_json::to_string_pretty(&objects)?;
        log::info!("updated_tokens: {}", updated_contents);

        let mut file = File::create(file_path)?;
        file.write_all(updated_contents.as_bytes())?;

        return Ok(self.token);
    }
}

pub fn check_token(given_token: &str) -> Result<(), std::io::Error> {
    let file_path = "/home/pi/.config/ledcat/token.txt";

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut token_valid = false;

    let tokens: Vec<Token> = serde_json::from_str(&contents)?;
    for token in tokens.iter() {
        if token.token == given_token.to_string() && !date_is_passed(&token.expiration_date) {
            token_valid = true;
            // break;
        }
    }
    log::info!("token_valid_end: {}", token_valid);
    if token_valid {
        return Ok(());
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "unvild token",
    ));
}

fn date_is_passed(date_str: &str) -> bool {
    let parsed_date = DateTime::parse_from_rfc3339(date_str)
        .expect("Failed to parse date string")
        .with_timezone(&Utc);

    // Get the current date and time
    let current_date = Utc::now();

    // Check if the parsed date is in the past
    return parsed_date < current_date;
}
