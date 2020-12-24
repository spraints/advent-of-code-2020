use std::env;

mod common;
mod day12;
mod day15;
mod day17;
mod day18;
mod day19;
mod day23;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        None => println!("Usage: cargo run dayN"),
        Some(arg) => match arg.as_str() {
            "day12" => day12::run(),
            "day15" => day15::run(),
            "day17" => day17::run(),
            "day18" => day18::run(),
            "day19" => day19::run(),
            "day23" => day23::run(),
            _ => println!("error: '{}' is not a known program", arg),
        },
    };
}
