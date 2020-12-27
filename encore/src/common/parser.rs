pub struct Parser<'a> {
    next: Option<char>,
    ch: std::str::Chars<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut ch = s.chars();
        let next = ch.next();
        Self { ch, next }
    }

    pub fn parse_usize(&mut self) -> usize {
        let mut res = 0;
        loop {
            match self.next {
                None => return res,
                Some(c) => match c.to_digit(10) {
                    None => return res,
                    Some(d) => res = res * 10 + (d as usize),
                },
            };
            self.next = self.ch.next();
        }
    }

    pub fn parse_char(&mut self) -> Result<char, String> {
        let res = self.next;
        self.next = self.ch.next();
        res.ok_or("expected a char but there wasn't one".to_string())
    }

    pub fn expect(&mut self, c: char) -> Result<(), String> {
        match self.next {
            None => return Err(format!("expected {:?} but got nothing", c).to_string()),
            Some(nc) => if nc != c { return Err(format!("expected {:?} but got {:?}", c, nc).to_string()) },
        }
        self.next = self.ch.next();
        Ok(())
    }

    pub fn rest(self) -> &'a str {
        self.ch.as_str()
    }
}
