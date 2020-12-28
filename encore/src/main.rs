use std::env;
use std::io::stdin;

mod common;

//mod template;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
            "day5" => day5::run(stdin()),
            "day6" => day6::run(stdin()),
            "day7" => day7::run(stdin()),
            _ => println!("'{}' is not implemented.", arg),
        },
    };
}
