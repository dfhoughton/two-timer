extern crate two_timer;

use two_timer::{parse, Config};
use std::env;
use chrono::Local;

fn main() {
    let mut args = env::args();
    args.next();

    let parse_str = args.collect::<Vec<String>>().join(" ");

    let config = Config::new(Local::now())
        .default_to_past(false)
        .select_instant(true);

    let res = parse(&parse_str, config).unwrap();

    println!("start: {:?}, end: {:?}", res.0, res.1);
    println!("start_local: {:?}, end_local: {:?}", res.0.with_timezone(&Local), res.1.with_timezone(&Local));
}
