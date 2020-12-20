use super::common;
use std::io;

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
    let messages: Vec<String> = common::parse_lines().collect();
    println!("rules:{} messages:{}", rules.len(), messages.len());
}

impl std::str::FromStr for RuleLine {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.chars();
        let (rule_number, _ /* ':' */) = common::parse_i64(&mut line);
        line.next().unwrap(); // ' '
        let rule = line.as_str().parse().unwrap();
        Ok(Self(rule_number.unwrap() as RuleRef, rule))
    }
}

impl std::str::FromStr for Rule {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut state = RuleParseState::Init;
        for c in line.chars() {
            state = state.next(c);
        }
        Ok(state.res())
    }
}

enum RuleParseState {
    Init,
    Rule1Num1(RuleRef),
    Rule1Num1Done(RuleRef),
    Rule1Num2(RuleRef, RuleRef),
    Rule2Init(Vec<RuleRef>),
    StartLiteral,
    LiteralA,
    LiteralB,
}

impl RuleParseState {
    fn next(self, c: char) -> Self {
        println!("{:?}", c);
        match self {
            RuleParseState::Init => {
                if let Some(d) = c.to_digit(10) {
                    RuleParseState::Rule1Num1(d as RuleRef)
                } else if c == '"' {
                    RuleParseState::StartLiteral
                } else {
                    panic!("unknown {:?}", c)
                }
            }
            RuleParseState::StartLiteral => {
                match c {
                    'a' => RuleParseState::LiteralA,
                    'b' => RuleParseState::LiteralB,
                    _ => panic!("unknown literal {:?}", c),
                }
            },
            RuleParseState::Rule1Num1(z) => {
                if let Some(d) = c.to_digit(10) {
                    let z = z * 10 + (d as RuleRef);
                    RuleParseState::Rule1Num1(z)
                } else if c == ' ' {
                    RuleParseState::Rule1Num1Done(z)
                } else {
                    todo!()
                }
            },
            RuleParseState::Rule1Num1Done(z) => {
                if c == '|' {
                    RuleParseState::Rule2Init(vec![z])
                } else if let Some(d) = c.to_digit(10) {
                    RuleParseState::Rule1Num2(z, d as RuleRef)
                } else {
                    todo!()
                }
            }
            RuleParseState::Rule1Num2(_, _) => todo!(),
            RuleParseState::Rule2Init(_) => todo!(),
            RuleParseState::LiteralA => RuleParseState::LiteralA,
            RuleParseState::LiteralB => RuleParseState::LiteralB,
        }
    }

    fn res(self) -> Rule {
        match self {
            RuleParseState::LiteralA => Rule::A,
            RuleParseState::LiteralB => Rule::B,
            _ => todo!(),
        }
    }
}
