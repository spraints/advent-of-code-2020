use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    part2: bool,
    input: i64
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Output {
}

pub fn solve(input: Input) -> Result<Output, String> {
    Ok(Output{})
}
