use chrono::prelude::*;
use chrono::offset::LocalResult;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct DtNoHours {
	yy: i32,
	mm: u32,
	dd: u32
}

#[post("/time_unix_no_hms/<format>", format = "json", data = "<utc_time>")]
pub fn find_unix_time(format: &str, utc_time: Json<DtNoHours>) -> String {
	let json_data = utc_time.into_inner();

	match Utc.with_ymd_and_hms(
        json_data.yy, 
        json_data.mm, 
        json_data.dd, 0, 0, 0) {
	    LocalResult::Single(data) => match format {
			"millis" | "ms" => format!("{}", data.timestamp_millis()), 
            "nanos" | "ns" => format!("{}", data.timestamp_nanos()),
            "secs" | "s" => format!("{}", data.timestamp()),
            "micros" | "mus" => format!("{}", data.timestamp_micros()),
            _ => String::from("Unrecognized format!")
		},
		LocalResult::None => String::from("Failed to parse date from JSON fields!"),
        LocalResult::Ambiguous(_,_) => String::from("The time you passed in could be interpreted in multiple ways. This is not implemented for this API, although it will be soon!")
	}
}
