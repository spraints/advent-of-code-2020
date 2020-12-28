use super::common;
use std::io::Read;
use std::str::FromStr;

pub fn run<R: Read>(r: R) {
    let passports = merge_passports(common::parse_lines(r));
    println!(
        "part 1: {}",
        passports.iter().filter(|p| is_valid(p)).count()
    );
    println!(
        "part 2: {}",
        passports.iter().filter(|p| is_valid2(p)).count()
    );
}

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const HGT: &str = "hgt";
const HCL: &str = "hcl";
const ECL: &str = "ecl";
const PID: &str = "pid";

fn is_valid(passport: &Passport) -> bool {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    for (f, _) in passport {
        match f.as_str() {
            BYR => byr = true,
            IYR => iyr = true,
            EYR => eyr = true,
            HGT => hgt = true,
            HCL => hcl = true,
            ECL => ecl = true,
            PID => pid = true,
            _ => (),
        };
    }
    byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn is_valid2(passport: &Passport) -> bool {
    /*
     * byr (Birth Year) - four digits; at least 1920 and at most 2002.
     * iyr (Issue Year) - four digits; at least 2010 and at most 2020.
     * eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
     * hgt (Height) - a number followed by either cm or in:
     * If cm, the number must be at least 150 and at most 193.
     * If in, the number must be at least 59 and at most 76.
     * hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
     * ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
     * pid (Passport ID) - a nine-digit number, including leading zeroes.
     * cid (Country ID) - ignored, missing or not.
     */
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    for (f, v) in passport {
        match f.as_str() {
            BYR => byr = betw(v, 1920, 2002),
            IYR => iyr = betw(v, 2010, 2020),
            EYR => eyr = betw(v, 2020, 2030),
            HGT => hgt = is_valid_hgt(v),
            HCL => hcl = is_valid_hcl(v),
            ECL => ecl = is_valid_ecl(v),
            PID => pid = is_valid_pid(v),
            _ => (),
        };
    }
    byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn betw(v: &String, min: usize, max: usize) -> bool {
    let v = v.parse().unwrap();
    min <= v && v <= max
}

fn is_valid_hgt(v: &String) -> bool {
    let mut p = common::make_parser(v);
    let v = p.parse_usize();
    let p = p.rest();
    match p {
        "cm" => 150 <= v && v <= 193,
        "in" => 59 <= v && v <= 76,
        _ => false,
        //_ => panic!("not right: {:?}", p),
    }
}

fn is_valid_hcl(v: &String) -> bool {
    let mut v = v.chars();
    match v.next() {
        Some('#') => (),
        _ => return false,
    };
    for _ in 0..6 {
        match v.next() {
            Some(c) => {
                if !c.is_digit(16) {
                    return false;
                }
            }
            None => return false,
        };
    }
    match v.next() {
        None => true,
        _ => false,
    }
}

fn is_valid_ecl(v: &String) -> bool {
    v == "amb" || v == "blu" || v == "brn" || v == "gry" || v == "grn" || v == "hzl" || v == "oth"
}

fn is_valid_pid(v: &String) -> bool {
    let mut v = v.chars();
    for _ in 0..9 {
        match v.next() {
            Some(c) => {
                if !c.is_digit(10) {
                    return false;
                }
            }
            None => return false,
        };
    }
    match v.next() {
        None => true,
        _ => false,
    }
}

fn merge_passports(lines: Vec<Line>) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut passport = Passport::new();
    for line in lines {
        match line.line {
            None => {
                let p = passport;
                passport = Passport::new();
                passports.push(p);
            }
            Some(fields) => {
                for f in fields {
                    passport.push(f);
                }
            }
        };
    }
    passports.push(passport);
    passports
}

struct Line {
    line: Option<Vec<(String, String)>>,
}

type Passport = Vec<(String, String)>;

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = if s.len() < 2 {
            None
        } else {
            Some(
                s.trim()
                    .split(' ')
                    .map(|f| {
                        let mut parts = f.split(':');
                        let a = parts.next().unwrap().to_string();
                        let b = parts.next().unwrap().to_string();
                        (a, b)
                    })
                    .collect(),
            )
        };
        Ok(Self { line })
    }
}
