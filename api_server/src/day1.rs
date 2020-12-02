use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    part2: bool,
    numbers: Vec<u64>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Output {
    numbers: Vec<u64>,
    result: u64,
}

pub fn solve(input: Input) -> Result<Output, String> {
    match solve2(&input.numbers, 0, 0, if input.part2 { 2 } else { 1 }) {
        Some(x) => Ok(x),
        None => Err("no set of three numbers adds up to 2020".to_string()),
    }
}

fn solve2(numbers: &Vec<u64>, min_index: usize, sum: u64, levels: u8) -> Option<Output> {
    let max_index = numbers.len() - (levels as usize);
    for i in (min_index..max_index) {
        let n = numbers[i];
        let sum = sum + n;
        if levels == 0 {
            if sum == 2020 {
                return Some(Output {
                    numbers: vec![n],
                    result: n,
                });
            }
        } else if sum < 2020 {
            if let Some(x) = solve2(numbers, i + 1, sum, levels - 1) {
                return Some(x.and(n));
            }
        }
    }
    None
}

impl Output {
    fn and(mut self, n: u64) -> Self {
        self.numbers.insert(0, n);
        self.result *= n;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = Input {
            part2: false,
            numbers: vec![1721, 979, 366, 299, 675, 1456],
        };
        assert_eq!(
            Ok(Output {
                numbers: vec![1721, 299],
                result: 514579
            }),
            solve(input)
        );
    }

    #[test]
    fn test_part2() {
        let input = Input {
            part2: true,
            numbers: vec![1721, 979, 366, 299, 675, 1456],
        };
        assert_eq!(
            Ok(Output {
                numbers: vec![979, 366, 675],
                result: 241861950,
            }),
            solve(input)
        );
    }
}
