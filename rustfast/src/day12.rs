use super::common::{self, complex::Complex};
use std::str::FromStr;

#[derive(Debug)]
struct Inst {
    command: char,
    magnitude: u16,
}

impl FromStr for Inst {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.chars();
        let command = s.next().unwrap();
        let magnitude = s.as_str().parse().unwrap();
        Ok(Self { command, magnitude })
    }
}

const N: Complex = Complex {
    real: 0,
    imaginary: 1,
};
const S: Complex = Complex {
    real: 0,
    imaginary: -1,
};
const E: Complex = Complex {
    real: 1,
    imaginary: 0,
};
const W: Complex = Complex {
    real: -1,
    imaginary: 0,
};

const R: Complex = Complex {
    real: 0,
    imaginary: -1,
};
const L: Complex = Complex {
    real: 0,
    imaginary: 1,
};

pub fn run() {
    let mut p1_pos = common::complex_number(0, 0);
    let mut p1_dir = common::complex_number(1, 0);
    let mut p2_pos = common::complex_number(0, 0);
    let mut p2_way = common::complex_number(10, 1);

    for inst in common::parse_lines::<Inst>() {
        match inst.command {
            'N' => {
                p1_pos = p1_pos + N * inst.magnitude;
                p2_way = p2_way + N * inst.magnitude;
            }
            'S' => {
                p1_pos = p1_pos + S * inst.magnitude;
                p2_way = p2_way + S * inst.magnitude;
            }
            'E' => {
                p1_pos = p1_pos + E * inst.magnitude;
                p2_way = p2_way + E * inst.magnitude;
            }
            'W' => {
                p1_pos = p1_pos + W * inst.magnitude;
                p2_way = p2_way + W * inst.magnitude;
            }
            'R' => {
                for _ in 0..(inst.magnitude / 90) {
                    p1_dir = p1_dir * R;
                    p2_way = p2_way * R;
                }
            }
            'L' => {
                for _ in 0..(inst.magnitude / 90) {
                    p1_dir = p1_dir * L;
                    p2_way = p2_way * L;
                }
            }
            'F' => {
                p1_pos = p1_pos + p1_dir * inst.magnitude;
                p2_pos = p2_pos + p2_way * inst.magnitude;
            }
            _ => {
                panic!("unrecognized: {:?}", inst);
            }
        };
    }

    println!("part 1: {}", md(p1_pos));
    println!("part 2: {}", md(p2_pos));
}

fn md(c: Complex) -> i64 {
    c.real.abs() + c.imaginary.abs()
}
