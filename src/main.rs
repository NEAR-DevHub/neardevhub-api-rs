pub mod changelog;
pub mod db;
pub mod entrypoints;
pub mod nearblocks_client;
pub mod rpc_service;
pub mod types;

use chrono::DateTime;
use regex::Regex;

pub fn timestamp_to_date_string(timestamp: i64) -> String {
    // Convert the timestamp to a NaiveDateTime
    let datetime = DateTime::from_timestamp_nanos(timestamp);

    // Format the NaiveDateTime to a string in YYYY-MM-DD format
    datetime.format("%Y-%m-%d").to_string()
}

pub fn separate_number_and_text(s: &str) -> (Option<i32>, String) {
    let number_regex = Regex::new(r"\d+").unwrap();

    if let Some(matched) = number_regex.find(s) {
        let number_str = matched.as_str();
        let number = number_str.parse::<i32>().unwrap();
        let text = s.replacen(number_str, "", 1).trim().to_string();
        (Some(number), text)
    } else {
        (None, s.trim().to_string())
    }
}

#[rocket::launch]
fn rocket() -> _ {
    devhub_cache_api::rocket(None)
}
