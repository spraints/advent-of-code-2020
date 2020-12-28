use super::common;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::str::FromStr;

pub fn run<R: Read>(r: R) {
    let rules = ruleshash(common::parse_lines(r));
    println!("part 1: {}", solve1(&rules));
    println!("part 2: {}", solve2(&rules));
}

fn solve2(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut bag_sizes = HashMap::new();
    ubs(&mut bag_sizes, input, "shiny gold") - 1
}

fn ubs(
    bag_sizes: &mut HashMap<String, usize>,
    input: &HashMap<String, Vec<(usize, String)>>,
    bag: &str,
) -> usize {
    if let Some(res) = bag_sizes.get(bag) {
        return *res;
    }
    let res = 1 + match input.get(bag) {
        None => 0,
        Some(inner) => inner
            .iter()
            .map(|(count, inner)| count * ubs(bag_sizes, input, inner))
            .sum(),
    };
    bag_sizes.insert(bag.to_string(), res);
    res
}

fn solve1(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut in2out: HashMap<&str, Vec<&str>> = HashMap::new();
    for (outer, inner) in input {
        for (_, inner) in inner {
            //println!("+ {:?} can be in {:?}", inner, outer);
            let v = in2out.entry(inner).or_insert(Vec::new());
            v.push(outer);
        }
    }
    let mut possible_outer = HashSet::new();
    let mut check: Vec<&str> = vec!["shiny gold"];
    loop {
        match check.pop() {
            None => return possible_outer.len() - 1,
            Some(bt) => {
                if !possible_outer.contains(bt) {
                    //println!("{}:", bt);
                    possible_outer.insert(bt);
                    if let Some(any) = in2out.get(bt) {
                        for obt in any {
                            //println!("  - {}", obt);
                            check.push(obt);
                        }
                    }
                }
            }
        }
    }
}

fn ruleshash(rules: Vec<Rule>) -> HashMap<String, Vec<(usize, String)>> {
    rules.into_iter().map(|r| (r.outer, r.inner)).collect()
}

struct Rule {
    outer: String,
    inner: Vec<(usize, String)>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("parse {:?}", s);
        let mut parts = s.split(" contain ");
        let outer = parts.next().unwrap().replace(" bags", "");
        //println!("  outer: {:?}", outer);
        let inner = match parts.next().unwrap() {
            "no other bags." => Vec::new(),
            s => s.split(", ").map(parse_inner).collect(),
        };
        //println!("  inner: {:?}", inner);
        Ok(Rule { outer, inner })
    }
}

fn parse_inner(s: &str) -> (usize, String) {
    let mut p = common::make_parser(s);
    let count = p.parse_usize();
    p.expect(' ').unwrap();
    let label = p
        .rest()
        .replace(".", "")
        .replace(" bags", "")
        .replace(" bag", "");
    (count, label)
}
