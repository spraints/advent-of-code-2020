use core::ops::{Add, Mul};
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct Complex {
    pub real: i64,
    pub imaginary: i64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}+{}i)", self.real, self.imaginary)
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        println!("{} * {}", self, other);
        // (r1 + i1 i) * (r2 + i2 i)
        // (r1 * r2 + r1 * i2 * i + i1 * i * r2 + i1 * i2 * -1)
        // (r1 * r2 - i1 * i2) + (r1 * i2 + r2 * i1) i
        let res = Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + other.real * self.imaginary,
        };
        println!(" => {}", res);
        res
    }
}

impl Mul<u16> for Complex {
    type Output = Self;

    fn mul(self, other: u16) -> Self {
        Complex {
            real: self.real * other as i64,
            imaginary: self.imaginary * other as i64,
        }
    }
}
