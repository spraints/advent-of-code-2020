use super::common;

pub fn run() {
    println!(
        "part 1: {}",
        common::parse_lines().fold(0, |sum, line| sum + eval_part1(line))
    );
}

fn eval_part1(expr: Expr) -> i32 {
    0
}

enum Item {
    Num(i32),
    Expr(Expr),
    Plus,
    Star,
}

struct Expr {
    exprs: Vec<Item>,
}

impl std::str::FromStr for Expr {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, ()> {
        todo!()
    }
}
