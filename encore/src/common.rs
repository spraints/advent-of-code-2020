use std::str::FromStr;
use std::fmt::Debug;

pub fn parse_lines<T>() -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
    read_lines().map(|line| line.parse().unwrap()).collect()
}

pub fn read_lines() -> ReadLines {
    let stdin = std::io::stdin();
    ReadLines{stdin, done: false}
}

pub struct ReadLines {
    stdin: std::io::Stdin,
    done: bool,
}

impl Iterator for ReadLines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let mut line = String::new();
            match self.stdin.read_line(&mut line).unwrap() {
                0 => {self.done = true; None},
                _ => Some(line.trim().to_string())
            }
        }
    }
}
