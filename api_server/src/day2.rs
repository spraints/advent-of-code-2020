use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    lines: Vec<PasswordLine>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PasswordLine {
    policy: PasswordPolicy,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PasswordPolicy {
    min: u16,
    max: u16,
    c: char,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Output {
    valid: Vec<PasswordLine>,
    invalid: Vec<PasswordLine>,
}

pub fn solve(input: Input) -> Result<Output, String> {
    let (valid, invalid) = input.lines.into_iter().partition(|line| line.is_valid());
    Ok(Output { valid, invalid })
}

impl PasswordLine {
    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.policy.c)
            .count() as u16;
        count >= self.policy.min && count <= self.policy.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pwline(min: u16, max: u16, c: char, password: &'static str) -> PasswordLine {
        PasswordLine {
            policy: PasswordPolicy { min, max, c },
            password: password.to_string(),
        }
    }

    #[test]
    fn test_sample() {
        let input = Input {
            lines: vec![
                pwline(1, 3, 'a', "abcde"),
                pwline(1, 3, 'b', "cdefg"),
                pwline(2, 9, 'c', "ccccccccc"),
            ],
        };
        assert_eq!(
            Ok(Output {
                valid: vec![pwline(1, 3, 'a', "abcde"), pwline(2, 9, 'c', "ccccccccc"),],
                invalid: vec![pwline(1, 3, 'b', "cdefg"),],
            }),
            solve(input)
        );
    }
}
