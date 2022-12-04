#[macro_use] extern crate rocket;
extern crate chrono;

mod timestamp_routes;
mod utc_routes;
use timestamp_routes::{unix_time,unix_time_fmt,unix_time_utc_fmt};
use utc_routes::{get_utc_time};
#[get("/")]
fn index() -> String {
    "Hello World!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index, 
        unix_time, 
        unix_time_fmt, 
        unix_time_utc_fmt, 
        get_utc_time
    ])
}
