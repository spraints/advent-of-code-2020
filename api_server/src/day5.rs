use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    part2: bool,
    input: String,
}

#[derive(Serialize)]
pub struct Output {}

pub fn solve(input: Input) -> Result<Output, String> {
    Err("todo".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i(input: &str, part2: bool) -> Input {
        Input {
            part2,
            input: input.to_string(),
        }
    }

    #[test]
    fn part1() {
        let input = "todo";
        let output = solve(i(input, false));
        assert!(output.is_ok());
    }
}
