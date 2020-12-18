use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    part2: bool,
    input: String,
}

#[derive(Serialize)]
pub struct Output {
    part1: u32,
    part2: u32,
}

pub fn solve(input: Input) -> Result<Output, String> {
    let mut input = input.input.chars();
    let (mut part1, mut part2) = (0, 0);
    loop {
        match group(&mut input)? {
            None => return Ok(Output { part1, part2 }),
            Some((p1, p2)) => {
                part1 += p1;
                part2 += p2;
            }
        }
    }
}

fn group<I>(stream: &mut I) -> Result<Option<(u32, u32)>, String>
where
    I: Iterator<Item = char>,
{
    let (mut p1, mut p2) = (0, 0xffffffff);
    loop {
        match line(stream)? {
            Line::EOF => {
                return Ok(if p1 > 0 {
                    Some((count(p1, 0), count(p2, 0)))
                } else {
                    None
                })
            }
            Line::EOG => return Ok(Some((count(p1, 0), count(p2, 0)))),
            Line::Resp(b) => {
                p1 = p1 | b;
                p2 = p2 & b;
            }
        };
    }
}

enum Line {
    EOF,
    EOG,
    Resp(u32),
}

fn line<I>(stream: &mut I) -> Result<Line, String>
where
    I: Iterator<Item = char>,
{
    let mut bits = 0;
    loop {
        match stream.next() {
            None => {
                return Ok(if bits == 0 {
                    Line::EOF
                } else {
                    Line::Resp(bits)
                })
            }
            Some('\n') => {
                return Ok(if bits == 0 {
                    Line::EOG
                } else {
                    Line::Resp(bits)
                })
            }
            Some('a') => bits |= 0x01 << 0,
            Some('b') => bits |= 0x01 << 1,
            Some('c') => bits |= 0x01 << 2,
            Some('d') => bits |= 0x01 << 3,
            Some('e') => bits |= 0x01 << 4,
            Some('f') => bits |= 0x01 << 5,
            Some('g') => bits |= 0x01 << 6,
            Some('h') => bits |= 0x01 << 7,
            Some('i') => bits |= 0x01 << 8,
            Some('j') => bits |= 0x01 << 9,
            Some('k') => bits |= 0x01 << 10,
            Some('l') => bits |= 0x01 << 11,
            Some('m') => bits |= 0x01 << 12,
            Some('n') => bits |= 0x01 << 13,
            Some('o') => bits |= 0x01 << 14,
            Some('p') => bits |= 0x01 << 15,
            Some('q') => bits |= 0x01 << 16,
            Some('r') => bits |= 0x01 << 17,
            Some('s') => bits |= 0x01 << 18,
            Some('t') => bits |= 0x01 << 19,
            Some('u') => bits |= 0x01 << 20,
            Some('v') => bits |= 0x01 << 21,
            Some('w') => bits |= 0x01 << 22,
            Some('x') => bits |= 0x01 << 23,
            Some('y') => bits |= 0x01 << 24,
            Some('z') => bits |= 0x01 << 25,
            Some(c) => return Err(format!("unexpected input: {}", c).to_string()),
        }
    }
}

fn count(bf: u32, accum: u32) -> u32 {
    match bf {
        0 => accum,
        _ => count(bf >> 1, accum + (bf & 0x01)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i(input: &str) -> Input {
        Input {
            input: input.to_string(),
            part2: false,
        }
    }

    #[test]
    fn the_test() {
        let input = "abc\n\
                     \n\
                     a\n\
                     b\n\
                     c\n\
                     \n\
                     ab\n\
                     ac\n\
                     \n\
                     a\n\
                     a\n\
                     a\n\
                     a\n\
                     \n\
                     b";
        let output = solve(i(input)).unwrap();
        assert_eq!(11, output.part1);
        assert_eq!(6, output.part2);
    }
}
