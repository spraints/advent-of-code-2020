use super::common;
use std::collections::HashMap;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let lines: Vec<String> = common::read_lines(r).collect();
    let (part1, part2) = count(&lines);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn count(lines: &[String]) -> (usize, usize) {
    let mut any = 0;
    let mut all = 0;
    let mut cur = HashMap::new();
    let mut curpeople = 0;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            any += cur.len();
            for (_, count) in cur {
                if count == curpeople {
                    all += 1;
                }
            }
            curpeople = 0;
            cur = HashMap::new();
        } else {
            curpeople += 1;
            for c in line.chars() {
                let counter = cur.entry(c).or_insert(0);
                *counter += 1;
            }
        }
    }
    any += cur.len();
    for (_, count) in cur {
        if count == curpeople {
            all += 1;
        }
    }
    (any, all)
}
