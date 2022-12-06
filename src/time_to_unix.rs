use chrono::prelude::*;
use chrono::offset::LocalResult;
use rocket::serde::json::Json;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct DtNoHours {
    yy: i32,
    mm: u32,
    dd: u32
}

#[post("/time_unix_no_hms/<format>", format = "json", data = "<utc_time>")]
pub fn find_unix_time(format: &str, utc_time: Json<DtNoHours>) -> String {
    let json_data = utc_time.into_inner();

    let date = NaiveDate::from_ymd(json_data.yy, json_data.mm, json_data.dd);
    let unix_timestamp_functions = HashMap::new()
        .insert("millis", Utc.timestamp_millis)
        .insert("ms", Utc.timestamp_millis)
        .insert("nanos", Utc.timestamp_nanos)
        .insert("ns", Utc.timestamp_nanos)
        .insert("secs", Utc.timestamp)
        .insert("s", Utc.timestamp)
        .insert("micros", Utc.timestamp_micros)
        .insert("mus", Utc.timestamp_micros);

    match unix_timestamp_functions.get(format) {
        Some(func) => format!("{}", func(date)),
        None => String::from("Unrecognized format!")
    }
}
