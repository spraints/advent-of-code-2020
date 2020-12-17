use super::common;
use std::collections::HashMap;

pub fn run() {
    let init: Vec<i64> = common::read_csv_line();
    let mut recents = HashMap::new();
    let mut turn = 0;
    let mut last = -1;
    for n in init {
        if last != -1 {
            recents.insert(last, turn);
        }
        turn += 1;
        last = n;
    }
    loop {
        last = if let Some(n) = recents.get_mut(&last) {
            let res = turn - *n;
            *n = turn;
            res
        } else {
            recents.insert(last, turn);
            0
        };
        turn += 1;
        if turn == 2020 {
            println!("part 1: {}", last);
        } else if turn == 30_000_000 {
            println!("part 2: {}", last);
            break;
        }
    }
}
