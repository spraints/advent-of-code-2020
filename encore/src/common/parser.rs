pub struct Parser<'a> {
    s: &'a str,
    ch: std::iter::Peekable<std::str::Chars<'a>>,
    i: usize,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        let ch = s.chars().peekable();
        let i = 0;
        Self { ch, i, s }
    }

    pub fn parse_usize(&mut self) -> usize {
        let mut res = 0;
        loop {
            match self.next_digit() {
                None => return res,
                Some(d) => res = res * 10 + (d as usize),
            };
        }
    }

    fn next_digit(&mut self) -> Option<u32> {
        match self.ch.peek() {
            None => None,
            Some(c) => {
                if c.is_digit(10) {
                    self.next().and_then(|c| c.to_digit(10))
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_char(&mut self) -> Result<char, String> {
        self.next()
            .ok_or_else(|| "expected a char but there wasn't one".to_string())
    }

    pub fn expect(&mut self, c: char) -> Result<(), String> {
        match self.next() {
            None => Err(format!("expected {:?} but got nothing", c)),
            Some(nc) => {
                if nc != c {
                    Err(format!("expected {:?} but got {:?}", c, nc))
                } else {
                    Ok(())
                }
            }
        }
    }

    fn next(&mut self) -> Option<char> {
        self.i += 1;
        self.ch.next()
    }

    pub fn rest(self) -> &'a str {
        self.s.get(self.i..).unwrap()
    }
}
