use std::env;

mod common;

mod day1;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        None => println!("Usage: cargo run dayN"),
        Some(arg) => match arg.as_str() {
            "day1" => day1::run(),
            _ => println!("'{}' is not implemented.", arg),
        },
    };
}
