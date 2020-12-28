use std::env;
use std::io::stdin;
use std::time::Instant;

mod common;

//mod template;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let start = Instant::now();
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
            "day8" => day8::run(stdin()),
            "day9" => day9::run(stdin()),
            "day10" => day10::run(stdin()),
            _ => panic!("'{}' is not implemented.", arg),
        },
    };
    if std::env::var("TIMER").is_ok() {
        println!(
            "completed in {} milliseconds",
            1000.0 * start.elapsed().as_secs_f32()
        );
    }
}
