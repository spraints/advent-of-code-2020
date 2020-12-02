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

pub fn solve(mut input: Input) -> Result<Output, String> {
    if input.part2 {
        return solve2(input)
    }
    input.numbers.sort();
    let n = input.numbers.len();
    let (a, b) = find2020(input.numbers, 0, n - 1)?;
    Ok(Output {
        numbers: vec![a, b],
        result: a * b,
    })
}

fn solve2(input: Input) -> Result<Output, String> {
    for (a,b,c) in each3(input.numbers) {
        if a + b + c == 2020 {
            return Ok(Output{
                numbers: vec![a,b,c],
                result: a * b * c,
            })
        }
    }
    Err("no set of three numbers adds up to 2020".to_string())
}

fn each3(numbers: Vec<u64>) -> Each3 {
    Each3{
        numbers,
        i: 0,
        j: 0,
        k: 0,
    }
}

struct Each3{
    numbers: Vec<u64>,
    i: usize,
    j: usize,
    k: usize,
}

impl Iterator for Each3 {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.numbers.len() < 3 {
            return None
        }
        if self.k == 0 {
            // Init
            self.i = 0;
            self.j = 1;
            self.k = 2;
        } else {
            if self.k < self.numbers.len() - 1 {
                self.k += 1
            } else if self.j < self.numbers.len() - 2 {
                self.j += 1;
                self.k = self.j + 1;
            } else if self.i < self.numbers.len() - 3 {
                self.i += 1;
                self.j = self.i + 1;
                self.k = self.j + 1;
            } else {
                return None
            }
        }
        Some((self.numbers[self.i], self.numbers[self.j], self.numbers[self.k]))
    }
}

pub fn find2020(input: Vec<u64>, i1: usize, i2: usize) -> Result<(u64, u64), String> {
    if i1 >= i2 {
        return Err("no valid pair was found!".to_string());
    }
    let a = input[i1];
    let b = input[i2];
    let sum = a + b;
    if sum == 2020 {
        Ok((a, b))
    } else if sum < 2020 {
        find2020(input, i1 + 1, i2)
    } else {
        find2020(input, i1, i2 - 1)
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
                numbers: vec![299, 1721],
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
