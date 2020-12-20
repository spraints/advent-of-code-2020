use super::common;
use std::io;
use std::collections::HashMap;

type RuleRef = usize;

enum Rule {
    A,
    B,
    Refs(Vec<Vec<RuleRef>>),
}

struct RuleLine(RuleRef, Rule);

pub fn run() {
    let stdin = io::stdin();
    let rules: Vec<RuleLine> = common::parse_lines_before_break().collect();
    let rules: HashMap<_, _> = rules.into_iter().map(|RuleLine(a,b)| (a,b)).collect();
    let messages: Vec<String> = common::parse_lines().collect();

    println!("part 1: {}", messages.iter().filter(|msg| satisfies_rule(msg.trim(), &rules, 0)).count());
}

fn satisfies_rule(msg: &str, rules: &HashMap<RuleRef, Rule>, rule: RuleRef) -> bool {
    let rule = &rules[&rule];
    rule.satisfied(msg, rules, |rest| rest.len() == 0)
}

impl Rule {
    fn satisfied<F: Fn(&str) -> bool>(&self, msg: &str, rules: &HashMap<RuleRef, Rule>, f: F) -> bool {
        match self {
            Rule::A => Self::must_start_with(msg, 'a', f),
            Rule::B => Self::must_start_with(msg, 'b', f),
            Rule::Refs(opts) => opts.iter().any(|opt| Self::check_rec(msg, opt, 0, rules, |rest| f(rest))),
        }
    }

    fn check_rec<F: Fn(&str) -> bool>(msg: &str, opt: &Vec<RuleRef>, i: usize, rules: &HashMap<RuleRef, Rule>, f: F) -> bool {
        match opt.get(i) {
            None => f(msg),
            Some(rr) => {
                let rule = &rules[rr];
                rule.satisfied(msg, rules, |rest| Self::check_rec(msg, opt, i, rules, |rest| f(rest)))
            }
        }
    }

    fn must_start_with<F: Fn(&str) -> bool>(msg: &str, c: char, f: F) -> bool {
        let mut msg = msg.chars();
        match msg.next() {
            None => false,
            Some(cc) => if c == cc {
                f(msg.as_str())
            } else {
                false
            }
        }
    }
}

impl std::str::FromStr for RuleLine {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split(|c| c == ':');
        let rule_ref = parts.next().unwrap().parse().unwrap();
        let rule = parts.next().unwrap().parse().unwrap();
        Ok(RuleLine(rule_ref, rule))
    }
}

impl std::str::FromStr for Rule {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split(|c| c == ' ');
        assert_eq!("", parts.next().unwrap());
        let mut part = parts.next().unwrap();
        if part == "\"a\"" {
            return Ok(Self::A);
        }
        if part == "\"b\"" {
            return Ok(Self::B);
        }
        let mut opt = Vec::new();
        let mut res = Vec::new();
        loop {
            if part == "|" {
                res.push(opt);
                opt = Vec::new();
            } else {
                let val = part.parse().unwrap();
                opt.push(val);
            }
            match parts.next() {
                None => {
                    res.push(opt);
                    return Ok(Self::Refs(res));
                }
                Some(s) => {
                    part = s;
                }
            };
        }
    }
}
