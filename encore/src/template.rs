use super::common;
use std::io::Read;
use std::str::FromStr;

pub fn run<R: Read>(r: R) {
    let passports = common::parse_lines(r);
    println!("part 1: {}", solve(&passports));
}

fn solve(input: &Vec<Line>) -> usize {
    0
}

struct Line {
    n: usize,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = common::make_parser(s);
        let n = p.parse_usize();
        Ok(Self { n })
    }
}
