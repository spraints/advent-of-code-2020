use super::common;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let codez = common::parse_lines(r);
    let secret = solve(&codez);
    println!("part 1: {}", secret);
    println!("part 2: {}", solve2(&codez, secret));
}

fn solve2(input: &[u64], goal: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    while b < input.len() {
        let vals = input.get(a..b).unwrap();
        match vals.iter().sum::<u64>() {
            n if n < goal => b += 1,
            n if n > goal => a += 1,
            _ => {
                let min = vals.iter().min().unwrap();
                let max = vals.iter().max().unwrap();
                return min + max;
            }
        };
    }
    panic!("no continguous set found!");
}

fn solve(input: &[u64]) -> u64 {
    let mut input = input.iter();
    let mut seen: Vec<u64> = Vec::new();
    for _ in 0..25 {
        seen.push(*input.next().unwrap());
    }
    for n in input {
        seen.sort_unstable();
        match twosum(&seen, *n) {
            None => return *n,
            Some(_) => seen.push(*n),
        };
    }
    panic!("all good!");
}

fn twosum(numbers: &[u64], goal: u64) -> Option<(usize, usize)> {
    let mut a = 0;
    let mut b = numbers.len() - 1;
    while a < b {
        match numbers[a] + numbers[b] {
            sum if sum == goal => return Some((a, b)),
            sum if sum < goal => a += 1,
            _ => b -= 1,
        };
    }
    None
}
