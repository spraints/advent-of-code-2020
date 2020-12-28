use std::env;
use std::io::stdin;

mod common;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        None => println!("Usage: cargo run dayN"),
        Some(arg) => match arg.as_str() {
            "day1" => day1::run(stdin()),
            "day2" => day2::run(stdin()),
            "day3" => day3::run(stdin()),
            "day4" => day4::run(stdin()),
            _ => println!("'{}' is not implemented.", arg),
        },
    };
}
