use std::env;

use chrono::prelude::Local;
use chrono::TimeZone;

fn main() {

    let args: Vec<String> = env::args().collect();


    let dt = chrono::Local.timestamp_nanos(args[1].parse::<i64>().unwrap());

    println!("Local time: {}\n", Local::now());
    println!("Input time: {}\n", dt);
}
