use std::io;
use std::marker::PhantomData;
use std::str::FromStr;

pub mod complex;

pub fn complex_number(real: i64, imaginary: i64) -> complex::Complex {
    complex::Complex { real, imaginary }
}

pub fn parse_lines<T>() -> ParseLines<T> {
    ParseLines::<T> {
        phantom: PhantomData,
    }
}

pub struct ParseLines<T> {
    phantom: PhantomData<T>,
}

impl<T> Iterator for ParseLines<T>
where
    T: FromStr,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match io::stdin()
            .read_line(&mut line)
            .expect("error parsing input")
        {
            0 => None,
            _ => match line.trim().parse() {
                Err(_) => panic!("error parsing line {:?}", line),
                Ok(val) => Some(val),
            },
        }
    }
}
