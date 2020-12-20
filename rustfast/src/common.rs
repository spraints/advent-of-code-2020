use std::io;
use std::marker::PhantomData;
use std::str::FromStr;

pub mod complex;

pub fn complex_number(real: i64, imaginary: i64) -> complex::Complex {
    complex::Complex { real, imaginary }
}

pub fn read_csv_line<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim()
        .split(",")
        .map(|x| match x.parse() {
            Ok(n) => n,
            Err(_) => panic!("error parsing line {}", line),
        })
        .collect()
}

pub fn parse_i64(c: &mut std::str::Chars) -> (Option<i64>, Option<char>) {
    let mut any = false;
    let mut res = 0;
    loop {
        match c.next() {
            Some('0') => res *= 10,
            Some('1') => res = res * 10 + 1,
            Some('2') => res = res * 10 + 2,
            Some('3') => res = res * 10 + 3,
            Some('4') => res = res * 10 + 4,
            Some('5') => res = res * 10 + 5,
            Some('6') => res = res * 10 + 6,
            Some('7') => res = res * 10 + 7,
            Some('8') => res = res * 10 + 8,
            Some('9') => res = res * 10 + 9,
            x => {return (if any { Some(res) } else { None }, x);}
        }
        any = true;
    }
}

pub fn parse_lines_before_break<T>() -> ParseLines<T> {
    ParseLines::<T> {
        phantom: PhantomData,
        stop_at_break: true,
        done: false,
    }
}

pub fn parse_lines<T>() -> ParseLines<T> {
    ParseLines::<T> {
        phantom: PhantomData,
        stop_at_break: false,
        done: false,
    }
}

pub struct ParseLines<T> {
    phantom: PhantomData<T>,
    stop_at_break: bool,
    done: bool,
}

impl<T> Iterator for ParseLines<T>
where
    T: FromStr,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut line = String::new();
        match io::stdin()
            .read_line(&mut line)
            .expect("error parsing input")
        {
            0 => None,
            n => {
                if n == 1 && self.stop_at_break {
                    self.done = true;
                    return None;
                }
                match line.trim().parse() {
                    Err(_) => panic!("error parsing line {:?}", line),
                    Ok(val) => Some(val),
                }
            }
        }
    }
}
