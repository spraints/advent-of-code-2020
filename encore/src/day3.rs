use super::common;
use std::io::Read;
use std::str::FromStr;

pub fn run<R: Read>(r: R) {
    let path = common::parse_lines(r);
    println!("part 1: {}", count_trees(&path, 3, 1));

    println!("part 2: {}",
             count_trees(&path, 1,1) *
             count_trees(&path,3,1) *
             count_trees(&path,5,1) *
             count_trees(&path,7,1) *
             count_trees(&path,1,2));
}

struct Line {
    spaces: Vec<bool>,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spaces = s.trim().chars().map(|c| c == '#').collect();
        Ok(Self{spaces})
    }
}

fn count_trees(grid: &Vec<Line>, right: usize, down: usize) -> usize {
    let (mut row, mut col) = (0,0);
    let mut res = 0;
    while row < grid.len() {
        let line = &grid[row];
        if line.spaces[col % line.spaces.len()] {
            res += 1;
        }
        row += down;
        col += right;
    }
    res
}
