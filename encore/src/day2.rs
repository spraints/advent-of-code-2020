use super::common;
use std::io::Read;
use std::str::FromStr;

pub fn run<R: Read>(r: R) {
    let mut valid1 = 0;
    let mut valid2 = 0;
    for line in common::parse_lines(r) {
        if is_valid1(&line) {
            valid1 += 1;
        }
        if is_valid2(&line) {
            valid2 += 1;
        }
    }
    println!("part 1: {}", valid1);
    println!("part 2: {}", valid2);
}

struct Line {
    a: usize,
    b: usize,
    c: char,
    password: String,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = common::make_parser(s);
        let a = parser.parse_usize();
        parser.expect('-')?;
        let b = parser.parse_usize();
        parser.expect(' ')?;
        let c = parser.parse_char()?;
        parser.expect(':')?;
        let password = parser.rest().trim().to_string();
        Ok(Line { a, b, c, password })
    }
}

fn is_valid1(line: &Line) -> bool {
    let c = line.password.chars().filter(|c| c == &line.c).count();
    line.a <= c && c <= line.b
}

fn is_valid2(line: &Line) -> bool {
    let a = line.password.chars().nth(line.a - 1).unwrap_or('-');
    let b = line.password.chars().nth(line.b - 1).unwrap_or('-');
    (a == line.c) != (b == line.c)
}
