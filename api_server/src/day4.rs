use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    part2: bool,
    input: String,
}

#[derive(Serialize)]
pub struct Output {
    valid: usize,
    scan_result: Vec<Result<(), String>>,
}

type P = Result<(), String>;

trait Validator {
    fn check(&self, passport: &str) -> P;
}

pub fn solve(input: Input) -> Result<Output, String> {
    let part2 = input.part2;
    let input = input.input;
    let scan_result = if !part2 {
        let validator = RequiredFieldValidator {
            required_fields: vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"],
        };
        scan(input, validator)
    } else {
        scan(input, DetailedValidator {})
    };
    let valid = scan_result.iter().filter(|x| x.is_ok()).count();
    Ok(Output { valid, scan_result })
}

fn scan(input: String, validator: impl Validator) -> Vec<P> {
    input
        .split("\n\n")
        .map(|passport| validator.check(passport))
        .collect()
}

struct RequiredFieldValidator {
    required_fields: Vec<&'static str>,
}

impl Validator for RequiredFieldValidator {
    fn check(&self, passport: &str) -> P {
        let fields: Vec<&str> = passport
            .split(char::is_whitespace)
            .filter_map(|entry| entry.split(':').next())
            .collect();
        let missing_fields: Vec<&&str> = self
            .required_fields
            .iter()
            .filter(|rf| !fields.contains(rf))
            .collect();
        if missing_fields.len() == 0 {
            Ok(())
        } else {
            Err(format!("missing: {:?}", missing_fields))
        }
    }
}

struct DetailedValidator {
    //validate_content: bool,
}

impl Validator for DetailedValidator {
    fn check(&self, passport: &str) -> P {
        let mut byr = false; // (Birth Year) - four digits; at least 1920 and at most 2002.
        let mut iyr = false; // (Issue Year) - four digits; at least 2010 and at most 2020.
        let mut eyr = false; // (Expiration Year) - four digits; at least 2020 and at most 2030.
        let mut hgt = false; // (Height) - a number followed by either cm or in:
                             // If cm, the number must be at least 150 and at most 193.
                             // If in, the number must be at least 59 and at most 76.
        let mut hcl = false; // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let mut ecl = false; // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let mut pid = false; // (Passport ID) - a nine-digit number, including leading zeroes.
        //let mut cid = false; // (Country ID) - ignored, missing or not.

        for field in passport.split(char::is_whitespace) {
            let mut parts = field.split(':');
            match parts.next() {
                Some("byr") => byr = true,
                Some("iyr") => iyr = true,
                Some("eyr") => eyr = true,
                Some("hgt") => hgt = true,
                Some("hcl") => hcl = true,
                Some("ecl") => ecl = true,
                Some("pid") => pid = true,
                //Some("cid") => cid = true,
                _ => (),
            }
        }

        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            Ok(())
        } else {
            Err(format!("byr:{} iyr:{} eyr:{} hgt:{} hcl:{} ecl:{} pid:{}",
                        byr, iyr, eyr, hgt, hcl, ecl, pid).to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn prep_input(part2: bool, input: &'static str) -> Input {
        let input = input.to_string();
        Input { input, part2 }
    }

    #[test]
    fn test_part1() {
        let input = prep_input(
            false,
            "\
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
            byr:1937 iyr:2017 cid:147 hgt:183cm\n\
            \n\
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
            hcl:#cfa07d byr:1929\n\
            \n\
            hcl:#ae17e1 iyr:2013\n\
            eyr:2024\n\
            ecl:brn pid:760753108 byr:1931\n\
            hgt:179cm\n\
            \n\
            hcl:#cfa07d eyr:2025 pid:166559648\n\
            iyr:2011 ecl:brn hgt:59in\n\
        ",
        );
        let output = solve(input).unwrap();
        assert_eq!(2, output.valid);
    }

    #[test]
    fn test_part2() {
        let input = "eyr:1972 cid:100\n\
                     hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
                     \n\
                     iyr:2019\n\
                     hcl:#602927 eyr:1967 hgt:170cm\n\
                     ecl:grn pid:012533040 byr:1946\n\
                     \n\
                     hcl:dab227 iyr:2012\n\
                     ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
                     \n\
                     hgt:59cm ecl:zzz\n\
                     eyr:2038 hcl:74454a iyr:2023\n\
                     pid:3556412378 byr:2007";
        let output = solve(prep_input(true, input)).unwrap();
        assert_eq!(0, output.valid);

        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
                     hcl:#623a2f\n\
                     \n\
                     eyr:2029 ecl:blu cid:129 byr:1989\n\
                     iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
                     \n\
                     hcl:#888785\n\
                     hgt:164cm byr:2001 iyr:2015 cid:88\n\
                     pid:545766238 ecl:hzl\n\
                     eyr:2022\n\
                     \n\
                     iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let output = solve(prep_input(true, input)).unwrap();
        assert_eq!(4, output.valid);
    }
}
