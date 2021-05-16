extern crate two_timer;

use two_timer::{parse, Config};
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let parse_str = args.collect::<Vec<String>>().join(" ");

    let config = Config::new().default_to_past(false);

    println!("{:?}", parse(&parse_str, Some(config)));
}
