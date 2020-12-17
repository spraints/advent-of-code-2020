use std::env;

mod common;
mod day12;
mod day15;
mod day17;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        None => println!("Usage: cargo run dayN"),
        Some(arg) => match arg.as_str() {
            "day12" => day12::run(),
            "day15" => day15::run(),
            "day17" => day17::run(),
            _ => println!("error: '{}' is not a known program", arg),
        },
    };
}
