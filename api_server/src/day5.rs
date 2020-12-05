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

#[derive(Debug, PartialEq)]
struct Seat {
    row: u16,
    seat: u16,
    id: u16,
}

fn seat(pass: &str) -> Result<Seat, String> {
    let mut c = pass.chars();
    let row = bits(&mut c, 0, 7, 'F', 'B')?;
    let seat = bits(&mut c, 0, 3, 'L', 'R')?;
    let id = seat + row * 8;
    Ok(Seat { row, seat, id })
}

fn bits<'a>(
    c: &mut std::str::Chars<'a>,
    res: u16,
    count: u8,
    zero: char,
    one: char,
) -> Result<u16, String> {
    match count {
        0 => Ok(res),
        _ => {
            let b = bit(c.next(), zero, one)?;
            bits(c, res * 2 + b, count - 1, zero, one)
        }
    }
}

fn bit(c: Option<char>, zero: char, one: char) -> Result<u16, String> {
    match c {
        None => Err("boarding pass is too short!".to_string()),
        Some(c) => {
            println!("C: {} [{}/{}]", c, zero, one);
            if c == zero {
                Ok(0)
            } else if c == one {
                Ok(1)
            } else {
                Err(format!("expected {} or {} but got {}", zero, one, c).to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //fn i(input: &str, part2: bool) -> Input {
    //    Input{part2, input: input.to_string()}
    //}

    #[test]
    fn part1() {
        assert_eq!(
            Seat {
                row: 70,
                seat: 7,
                id: 567
            },
            seat("BFFFBBFRRR").unwrap()
        );
        assert_eq!(
            Seat {
                row: 14,
                seat: 7,
                id: 119
            },
            seat("FFFBBBFRRR").unwrap()
        );
        assert_eq!(
            Seat {
                row: 102,
                seat: 4,
                id: 820
            },
            seat("BBFFBBFRLL").unwrap()
        );
    }
}
