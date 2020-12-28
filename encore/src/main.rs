use std::env;
use std::fs::File;
use std::io::stdin;
use std::time::Instant;

mod common;

//mod template;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        None => println!("Usage: cargo run dayN"),
        Some(arg) => match arg.as_str() {
            "timed" => time_all(),
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
            "day11" => day11::run(stdin()),
            _ => panic!("'{}' is not implemented.", arg),
        },
    };
}

fn time_all() {
    let start = Instant::now();
    time(day1::run, "../fast/in/1");
    time(day2::run, "../fast/in/2");
    time(day3::run, "../fast/in/3");
    time(day4::run, "../fast/in/4");
    time(day5::run, "../fast/in/5");
    time(day6::run, "../fast/in/6");
    time(day7::run, "../fast/in/7");
    time(day8::run, "../fast/in/8");
    time(day9::run, "../fast/in/9");
    time(day10::run, "../fast/in/10");
    time(day11::run, "../fast/in/11");
    println!("total time: {} ms", 1000.0 * start.elapsed().as_secs_f32());
}

fn time<F: FnOnce(File)>(f: F, path: &str) {
    let start = Instant::now();
    f(File::open(path).unwrap());
    println!("{}: {} ms", path, 1000.0 * start.elapsed().as_secs_f32());
}
