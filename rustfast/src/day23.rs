const INPUT: &str = "598162734";

pub fn run() {
    let now = std::time::Instant::now();
    part1();
    let part1_time = now.elapsed();
    let now = std::time::Instant::now();
    part2();
    let part2_time = now.elapsed();
    println!("times: {} / {} ms", part1_time.as_millis(), part2_time.as_millis());
}

fn part1() {
    let res = play(9, 100);
    let mut n = 1;
    print!("part 1: ");
    loop {
        let m = res[n];
        if m == 1 {
            break;
        }
        print!("{}", m);
        n = m;
    }
    println!("");
}

fn part2() {
    let res = play(1_000_000, 10_000_000);
    let a = res[1];
    let b = res[a];
    println!("part 2: {}", a * b);
}

fn play(max: usize, rounds: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(max);
    let first = INPUT.chars().next().unwrap().to_digit(10).unwrap() as usize;
    let mut last = None;
    for c in INPUT.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        if let Some(last) = last.replace(digit) {
            if last < res.len() {
                res[last] = digit;
            } else {
                res.resize(last + 1, digit);
            }
        };
    }
    let mut last = last.unwrap();
    while res.len() <= max {
        let i = res.len();
        if last < res.len() {
            res[last] = i;
        } else {
            res.resize(last + 1, i);
        }
        last = i;
    }
    res[last] = first;

    let mut current = first;
    for _ in (0..rounds) {
        let pick1 = res[current];
        let pick2 = res[pick1];
        let pick3 = res[pick2];
        let rest = res[pick3];
        let mut dest = current - 1;
        loop {
            if dest < 1 {
                dest = max;
            }
            if pick1 == dest || pick2 == dest || pick3 == dest {
                dest -= 1;
            } else {
                break;
            }
        }
        res[current] = rest;
        res[pick3] = res[dest];
        res[dest] = pick1;
        current = rest;
    }
    res
}
