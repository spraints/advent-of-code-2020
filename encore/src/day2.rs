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
    a: u32,
    b: u32,
    c: char,
    password: String,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.chars();
        let mut c = s.next().unwrap();
        let mut a = 0;
        while c.is_digit(10) {
            a = a * 10 + c.to_digit(10).unwrap();
            c = s.next().unwrap();
        }
        assert_eq!('-', c);
        c = s.next().unwrap();
        let mut b = 0;
        while c.is_digit(10) {
            b = b * 10 + c.to_digit(10).unwrap();
            c = s.next().unwrap();
        }
        assert_eq!(' ', c);
        let c = s.next().unwrap();
        assert_eq!(Some(':'), s.next());
        let password = s.as_str().trim().to_string();
        Ok(Line { a, b, c, password })
    }
}

fn is_valid1(line: &Line) -> bool {
    let c = line.password.chars().filter(|c| c == &line.c).count() as u32;
    line.a <= c && c <= line.b
}

fn is_valid2(line: &Line) -> bool {
    let a = line
        .password
        .chars()
        .nth((line.a - 1) as usize)
        .unwrap_or('-');
    let b = line
        .password
        .chars()
        .nth((line.b - 1) as usize)
        .unwrap_or('-');
    (a == line.c) != (b == line.c)
}
