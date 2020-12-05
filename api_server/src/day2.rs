use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    input: Vec<PasswordLine>,
    part2: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PasswordLine {
    policy: PasswordPolicy,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    c: char,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Output {
    valid: Vec<PasswordLine>,
    invalid: Vec<PasswordLine>,
}

pub fn solve(input: Input) -> Result<Output, String> {
    let part2 = input.part2;
    let (valid, invalid) = input
        .input
        .into_iter()
        .partition(|line| line.is_valid(part2));
    Ok(Output { valid, invalid })
}

impl PasswordLine {
    fn is_valid(&self, part2: bool) -> bool {
        if part2 {
            let chars: Vec<char> = self.password.chars().collect();
            match chars.get(self.policy.min - 1) {
                None => false,
                Some(c1) => match chars.get(self.policy.max - 1) {
                    None => *c1 == self.policy.c,
                    Some(c2) => {
                        if *c1 == self.policy.c {
                            *c2 != self.policy.c
                        } else {
                            *c2 == self.policy.c
                        }
                    }
                },
            }
        } else {
            let count = self
                .password
                .chars()
                .filter(|c| *c == self.policy.c)
                .count();
            count >= self.policy.min && count <= self.policy.max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pwline(min: usize, max: usize, c: char, password: &'static str) -> PasswordLine {
        PasswordLine {
            policy: PasswordPolicy { min, max, c },
            password: password.to_string(),
        }
    }

    #[test]
    fn test_sample() {
        let input = Input {
            part2: false,
            input: vec![
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

    #[test]
    fn test_part2() {
        let input = Input {
            part2: true,
            input: vec![
                pwline(1, 3, 'a', "abcde"),
                pwline(1, 3, 'b', "cdefg"),
                pwline(2, 9, 'c', "ccccccccc"),
            ],
        };
        assert_eq!(
            Ok(Output {
                valid: vec![pwline(1, 3, 'a', "abcde")],
                invalid: vec![pwline(1, 3, 'b', "cdefg"), pwline(2, 9, 'c', "ccccccccc")],
            }),
            solve(input)
        );
    }
}
