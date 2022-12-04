#[macro_use] extern crate rocket;
extern crate chrono;
mod time_to_unix;
mod timestamp_routes;
mod utc_routes;
use time_to_unix::{find_unix_time};
use timestamp_routes::{unix_time,unix_time_fmt,unix_time_utc_fmt};
use utc_routes::{get_utc_time};
use rocket::shield::Shield;
use rocket::shield::XssFilter;

#[get("/")]
fn index() -> String {
    "Hello World!".to_string()
}

#[launch]
fn rocket() -> _ {
    let shield_config: Shield = Shield::default().enable(XssFilter::default());
    rocket::build().attach(shield_config).mount("/", routes![
        index, 
        unix_time, 
        unix_time_fmt, 
        unix_time_utc_fmt, 
        get_utc_time,
        find_unix_time
    ])
}
