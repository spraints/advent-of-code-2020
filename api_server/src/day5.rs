use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    part2: bool,
    input: Vec<String>,
}

#[derive(Serialize)]
pub struct Output {
    max_id: u16,
    my_seat: u16,
    seats: Vec<Seat>,
}

#[derive(Serialize, Debug, PartialEq)]
struct Seat {
    row: u16,
    seat: u16,
    id: u16,
}

pub fn solve(input: Input) -> Result<Output, String> {
    let mut seats = Vec::<Seat>::new();
    let mut max_id = 0;
    for pass in input.input {
        let seat = seat(pass.as_str())?;
        if seat.id > max_id {
            max_id = seat.id;
        }
        seats.push(seat);
    }
    let my_seat = find_my_seat(&seats, max_id);
    Ok(Output { max_id, my_seat, seats })
}

fn find_my_seat(seats: &Vec<Seat>, max_id: u16) -> u16 {
    let mut occupied = [false; 128*8];
    for seat in seats {
        occupied[seat.id as usize] = true;
    }
    let mut last2 = (false, false);
    for i in 0..max_id {
        let cur = occupied[i as usize];
        match (last2, cur) {
            ((true, false), true) => return i - 1,
            _ => ()
        };
        last2 = (last2.1, cur);
    }
    0
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

    #[test]
    fn part2() {
        let seats = vec![
            Seat{row:0, seat:0, id:193},
            Seat{row:0, seat:0, id:191},
Seat{row:0, seat:0, id:194}
        ];
        assert_eq!(192, find_my_seat(&seats, 300));
    }
}
