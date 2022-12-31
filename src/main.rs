use chrono::prelude::Local;
use chrono::TimeZone;
use chrono::Timelike;
use clap::crate_authors;
use clap::crate_description;
use clap::crate_version;
use clap::{Arg, Command};

fn main() {
    let arguments = Command::new("Nanotots")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("timestamp_nanos")
                .short('n')
                .long("nano")
                .value_name("timestamp")
                .help("Convert a unix timestamp to human time with nanosecond precision")
                .takes_value(true),
        )
        .arg(
            Arg::new("human_time")
                .short('l')
                .long("human")
                .value_name("human_time")
                .takes_value(true)
                .max_values(6)
                .min_values(1)
                .help("Convert human time {YYYY MM DD HH mm ss} to a timestamp "),
        )
        .get_matches();

    if arguments.is_present("human_time") {
        println!("Need to show human time!!");

        let possible_dates = arguments.values_of("human_time").unwrap();

        let mut year: usize = 0;
        let mut month: usize = 0;
        let mut day: usize = 0;

        let mut hour: usize = 0;
        let mut minute: usize = 0;
        let mut second: usize = 0;

        for (index, val) in possible_dates.enumerate() {
            match index {
                0 => year = val.parse().unwrap_or(0),
                1 => month = val.parse().unwrap_or(0),
                2 => day = val.parse().unwrap_or(0),
                3 => hour = val.parse().unwrap_or(0),
                4 => minute = val.parse().unwrap_or(0),
                5 => second = val.parse().unwrap_or(0),
                _ => {
                    println!("Too many ARGS!!")
                }
            }
        }

        // can maybe add something to check for out of range numbers? Chrono also does it too so
        // maybe not needed...

        // formatting magic to pad numbers
        let rfc3339 = format!(
            "{:0>2}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}+02:00",
            year, month, day, hour, minute, second
        );

        println!("Parsed date to be converted: \n{}", rfc3339);

        let stamp = chrono::DateTime::parse_from_rfc3339(&rfc3339).unwrap();

        println!("Parsed UNIX timestamp: \n{:?}", stamp.timestamp());

        // Print Local time and TS
        println!(
            "Local time: {} -- {}:{}:{} \nTS: {}",
            Local::now().date(),
            Local::now().hour(),
            Local::now().minute(),
            Local::now().second(),
            Local::now().timestamp(),
        );
    } else if arguments.is_present("timestamp_nanos") {
        let stamp = arguments.value_of("timestamp_nanos").unwrap();
        let dt = chrono::Local.timestamp_nanos(stamp.parse().unwrap());

        println!(
            "Local time: {} -- {}:{}:{} \n",
            Local::now().date(),
            Local::now().hour(),
            Local::now().minute(),
            Local::now().second()
        );
        println!(
            "Input time: {} -- {}:{}:{} \n",
            dt.date(),
            dt.hour(),
            dt.minute(),
            dt.second()
        );
    } else {
        println!("No valid arg passed!");
    }
}
