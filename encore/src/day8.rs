use super::common;
use std::collections::HashSet;
use std::io::Read;
use std::str::FromStr;

pub fn run<R: Read>(r: R) {
    let insts = common::parse_lines(r);
    println!("part 1: {}", run_program(&insts));
    println!("part 2: {}", solve2(insts));
}

fn solve2(insts: Vec<Line>) -> i64 {
    let insts = flipone(insts);
    run_program(&insts)
}

fn flipone(mut insts: Vec<Line>) -> Vec<Line> {
    let graph = graphit(&insts);
    let g1 = from_beginning(&graph);
    let g2 = from_end(&graph);
    for i in g1 {
        let (_, k) = graph[i];
        if g2.contains(&k) {
            insts[i] = mutinst(&insts[i]);
            return insts;
        }
    }
    panic!("no path out found!");
}

fn from_beginning(graph: &[(usize, usize)]) -> HashSet<usize> {
    let graph: Vec<Vec<usize>> = graph.iter().map(|n| vec![n.0]).collect();
    walk(&graph, 0)
}

fn from_end(graph: &[(usize, usize)]) -> HashSet<usize> {
    let mut reversed = Vec::new();
    reversed.resize_with(graph.len() + 1, || Vec::new());
    for (i, (nxt, _)) in graph.iter().enumerate() {
        if let Some(r) = reversed.get_mut(*nxt) {
            r.push(i);
        }
    }
    walk(&reversed, graph.len())
}

fn walk(graph: &[Vec<usize>], start: usize) -> HashSet<usize> {
    let mut to_visit = vec![start];
    let mut res = HashSet::new();
    loop {
        match to_visit.pop() {
            None => return res,
            Some(i) => {
                if res.insert(i) {
                    for nxt in &graph[i] {
                        to_visit.push(*nxt);
                    }
                }
            }
        };
    }
}

fn mutinst(i: &Line) -> Line {
    let inst = if i.inst == "jmp" { "nop" } else { "jmp" }.to_string();
    Line { inst, arg: i.arg }
}

fn graphit(insts: &[Line]) -> Vec<(usize, usize)> {
    insts
        .iter()
        .enumerate()
        .map(|(i, inst)| match inst.inst.as_str() {
            "acc" => (i + 1, i + 1),
            "jmp" => (jmp(i, inst.arg), i + 1),
            "nop" => (i + 1, jmp(i, inst.arg)),
            _ => panic!("boom"),
        })
        .collect()
}

fn run_program(input: &[Line]) -> i64 {
    let mut acc = 0;
    let mut pc = 0;
    let mut pcs = HashSet::new();
    while pc < input.len() && !pcs.contains(&pc) {
        pcs.insert(pc);
        let inst = &input[pc];
        match inst.inst.as_str() {
            "acc" => acc += inst.arg,
            "nop" => (),
            "jmp" => pc = jmp(pc, inst.arg) - 1,
            _ => panic!("unexpected instruction {:?}", inst.inst),
        };
        pc += 1;
    }
    acc
}

fn jmp(pc: usize, off: i64) -> usize {
    (pc as i64 + off) as usize
}

struct Line {
    inst: String,
    arg: i64,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = common::make_parser(s);
        let inst = p.parse_word();
        p.expect(' ')?;
        let mul = match p.parse_char() {
            Ok('+') => 1,
            Ok('-') => -1,
            c => return Err(format!("expected + or - but got {:?}", c)),
        };
        let arg = mul * (p.parse_usize() as i64);
        Ok(Self { inst, arg })
    }
}
