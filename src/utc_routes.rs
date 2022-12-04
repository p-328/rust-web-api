use chrono::prelude::*;
#[get("/utc")]
pub fn get_utc_time() -> String {
	format!("{}", Utc::now().format("%Y-%m-%dT%H:%M:%S"))
}