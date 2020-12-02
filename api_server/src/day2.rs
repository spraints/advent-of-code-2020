use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize, Debug, PartialEq)]
pub struct Output {}

pub fn solve(mut input: Input) -> Result<Output, String> {
    Err("todo".to_string())
}
