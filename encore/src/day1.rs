use super::common;

pub fn run() {
    let mut numbers = common::parse_lines();
    numbers.sort();
    let (a,b) = twosum(&numbers, 2020).unwrap();
    println!("part 1: {}", numbers[a] * numbers[b]);
    let (a,b,c) = threesum(&numbers, 2020).unwrap();
    println!("part 2: {}", numbers[a] * numbers[b] * numbers[c]);
}

fn threesum(numbers: &[i64], goal: i64) -> Option<(usize, usize, usize)> {
    let mut a = numbers.len();
    while a > 0 {
        a -= 1;
        match twosum(&numbers[..a], goal-numbers[a]) {
            None => (),
            Some((b, c)) => return Some((a,b,c)),
        };
    }
    None
}

fn twosum(numbers: &[i64], goal: i64) -> Option<(usize, usize)> {
    let mut a = 0;
    let mut b = numbers.len() - 1;
    while a < b {
        let sum = numbers[a] + numbers[b];
        if sum == goal {
            return Some((a, b));
        } else if sum < goal {
            a += 1;
        } else {
            b -= 1;
        }
    }
    None
}
