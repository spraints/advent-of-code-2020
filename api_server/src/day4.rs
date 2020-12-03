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
    use pretty_assertions::assert_eq;

    // #[test]
    // fn test_part1() {
    //     panic!("todo");
    // }

    // #[test]
    // fn test_part2() {
    //     panic!("todo");
    // }
}
