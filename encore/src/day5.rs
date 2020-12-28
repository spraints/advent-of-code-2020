use super::common;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut passes: Vec<usize> = common::read_lines(r).map(parse_pass).collect();
    println!("part 1: {}", passes.iter().max().unwrap());
    passes.sort_unstable();
    for (i, val) in passes.iter().enumerate() {
        if val + 1 != passes[i + 1] {
            println!("part 2: {}", val + 1);
            break;
        }
    }
}

fn parse_pass(s: String) -> usize {
    s.chars().fold(0, |res, c| {
        res * 2 + if c == 'B' || c == 'R' { 1 } else { 0 }
    })
}
