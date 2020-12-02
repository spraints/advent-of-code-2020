use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    numbers: Vec<u64>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Output {
    a: u64,
    b: u64,
    result: u64,
}

pub fn solve(mut input: Input) -> Result<Output, String> {
    input.numbers.sort();
    let n = input.numbers.len();
    let (a, b) = find2020(input.numbers, 0, n - 1)?;
    Ok(Output {
        a,
        b,
        result: a * b,
    })
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
            numbers: vec![1721, 979, 366, 299, 675, 1456],
        };
        assert_eq!(
            Ok(Output {
                a: 299,
                b: 1721,
                result: 514579
            }),
            solve(input)
        );
    }
}
