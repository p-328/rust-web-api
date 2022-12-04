use std::time::{SystemTime, Duration};
use rocket::serde::json::Json;
use serde::Deserialize;
use chrono::prelude::*;


#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UnixSecondsJson {
    pub unix_time: u64
}
// TODO: Implement proper UTC format.
fn utc_time(unix_secs: u64) -> String {
    let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(
                i64::try_from(unix_secs).unwrap(), 0), Utc);
    format!("{}", datetime.format("%Y-%m-%dT%H:%M:%S"))
}

#[get("/unix_tstamp")]
pub fn unix_time() -> String {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(e) => format!("{}", e.as_secs()),
        Err(_) => String::from("Could not load Unix Time!")
    }
}

#[get("/unix_tstamp/<format>")]
pub fn unix_time_fmt(format: &str) -> String {
    match format {
        "nanos" | "ns" =>
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(e) => format!("{}", e.as_nanos()),
                Err(_) => String::from("Error loading Unix Time!")
            },
        "millis" | "ms" =>
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(e) => format!("{}", e.as_millis()),
                Err(_) => String::from("Error loading Unix Time!")
            }
        "secs" | "s" =>
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(e) => format!("{}", e.as_secs()),
                Err(_) => String::from("Error loading Unix Time!")
            },
        "micros" | "mus" => 
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(e) => format!("{}", e.as_micros()),
                Err(_) => String::from("Error loading Unix Time!")
            },
        _ => String::from("Unrecognized format")
    }
}
// TODO: Make proper UTC time format
#[post("/unix_tstamp/<format>/utc", format = "json", data = "<unix_time>")]
pub fn unix_time_utc_fmt(format: &str, unix_time: Json<UnixSecondsJson>) -> String {
    match format {
        "nanos" | "ns" => {
            let unix_time_parsed = Duration::from_nanos(
                unix_time.into_inner().unix_time).as_secs();
            utc_time(unix_time_parsed)
        },
        "millis" | "ms" => {
            let unix_time_parsed = Duration::from_millis(
                unix_time.into_inner().unix_time).as_secs();
            utc_time(unix_time_parsed)
        },
        "secs" | "s" => {
            let unix_time_parsed = Duration::from_millis(
                unix_time.into_inner().unix_time).as_secs();
            utc_time(unix_time_parsed)
        }
        _ => "Unrecognized format!".to_string()
    }
}