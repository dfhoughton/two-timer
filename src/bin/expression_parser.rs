extern crate two_timer;

use two_timer::{parse, Config};
use std::env;
use chrono_tz::US::Pacific;
use chrono::Local;

fn main() {
    let mut args = env::args();
    args.next();

    let parse_str = args.collect::<Vec<String>>().join(" ");

    let config = Config::new(Local::now().with_timezone(&Pacific)).default_to_past(false);

    let res = parse(&parse_str, config).unwrap();

    println!("start: {:?}, end: {:?}", res.0, res.1);
    println!("start_local: {:?}, end_local: {:?}", res.0.with_timezone(&Local), res.1.with_timezone(&Local));
}
