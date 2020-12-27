use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub fn parse_lines<T, R>(r: R) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    R: Read,
{
    read_lines(r).map(|line| line.parse().unwrap()).collect()
}

pub fn read_lines<R: Read>(r: R) -> ReadLines<BufReader<R>> {
    ReadLines {
        reader: BufReader::new(r),
        done: false,
    }
}

pub struct ReadLines<R> {
    reader: R,
    done: bool,
}

impl<R: BufRead> Iterator for ReadLines<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let mut line = String::new();
            match self.reader.read_line(&mut line).unwrap() {
                0 => {
                    self.done = true;
                    None
                }
                _ => Some(line.trim().to_string()),
            }
        }
    }
}
