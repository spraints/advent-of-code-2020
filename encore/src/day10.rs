use super::common;
use std::io::Read;
use std::collections::HashMap;

pub fn run<R: Read>(r: R) {
    let mut joltages = common::parse_lines(r);
    joltages.sort_unstable();
    joltages.push(joltages[joltages.len()-1] + 3);
    println!("part 1: {}", solve(&joltages));
    println!("part 2: {}", solve2(&joltages));
}

fn solve2(joltages: &Vec<u16>) -> usize {
    let mut memo = HashMap::new();
    2 * paths(0, &joltages, &mut memo)
}

fn paths(j: u16, joltages: &[u16], memo: &mut HashMap<u16, usize>) -> usize {
    if joltages.len() == 1 {
        return 1;
    }
    if let Some(n) = memo.get(&j) {
        return *n;
    }
    let max = j + 3;
    let mut res = 0;
    for i in 1..joltages.len() {
        let nxt = joltages[i];
        if nxt > max {
            break;
        }
        res += paths(nxt, &joltages[i..], memo);
    }
    memo.insert(j, res);
    res
}

fn solve(joltages: &Vec<u16>) -> usize {
    let mut s1 = 0;
    let mut s3 = 0;
    let mut j1 = &0;
    for (i, j2) in joltages.iter().enumerate() {
        //println!("[{}, {}] => {}", j1, j2, j2 - j1);
        match j2 - j1 {
            0 => panic!("did not expect a duplicate! {}", j1),
            1 => s1 += 1,
            3 => s3 += 1,
            _ => (),
        };
        j1 = j2;
    }
    //println!("{}, {}", s1, s3);
    s1*s3
}
