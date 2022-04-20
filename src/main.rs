use chrono::prelude::Local;
use chrono::TimeZone;
use clap::crate_authors;
use clap::crate_description;
use clap::crate_version;
use clap::{Arg, Command};

fn main() {
    let arguments = Command::new("Test nanotots")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("timestamp_nanos")
                .short('n')
                .long("nano")
                .value_name("TIMESTAMP_NANOS")
                .takes_value(true),
        )
        .get_matches();

    match arguments.occurrences_of("timestamp_nanos") {
        0 => println!("No args supplied"),
        1 => {
            let stamp = arguments.value_of("timestamp_nanos").unwrap();
            let dt = chrono::Local.timestamp_nanos(stamp.parse().unwrap());

            println!("Local time: {}\n", Local::now());
            println!("Input time: {}\n", dt);
        }
        _ => todo!(),
    }
}
