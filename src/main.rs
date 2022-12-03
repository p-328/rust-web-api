#[macro_use] extern crate rocket;
use std::time::SystemTime;

#[get("/")]
fn index() -> String {
    "Hello World!".to_string()
}

#[get("/unix_tstamp")]
fn unix_time() -> String {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(e) => format!("{}", e.as_secs()),
        Err(_) => String::from("Could not load Unix Time!")
    }
}

#[get("/unix_tstamp/<format>")]
fn unix_time_fmt(format: &str) -> String {
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


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, unix_time, unix_time_fmt])
}
