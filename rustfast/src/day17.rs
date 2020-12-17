use super::common;
use std::collections::HashMap;
use std::hash::Hash;
use std::slice::Iter;

trait Coords {
    fn offset(&self, d: i32) -> Self;
    fn upto(&self, other: &Self) -> Box<dyn Iterator<Item = Self>>;

    fn neighbors(&self) -> Box<dyn Iterator<Item = Self>> where Self: Sized {
        let start = self.offset(-1);
        let stop = self.offset(1);
        start.upto(&stop)
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Coords3(i32, i32, i32);

impl Coords for Coords3 {
    fn offset(&self, d: i32) -> Self {
        Self(self.0 + d, self.1 + d, self.2 + d)
    }

    fn upto(&self, other: &Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(Upto3 {
            start: self.clone(),
            stop: other.clone(),
            cursor: Self(self.0, self.1, self.2 - 1),
        })
    }
}

impl Coords3 {
    fn to4(&self) -> Coords4 {
        Coords4(self.0, self.1, self.2, 0)
    }
}

struct Upto3 {
    start: Coords3,
    stop: Coords3,
    cursor: Coords3,
}

impl Iterator for Upto3 {
    type Item = Coords3;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.2 = self.cursor.2 + 1;
        if self.cursor.2 > self.stop.2 {
            self.cursor.2 = self.start.2;
            self.cursor.1 = self.cursor.1 + 1;
        }
        if self.cursor.1 > self.stop.1 {
            self.cursor.1 = self.start.1;
            self.cursor.0 = self.cursor.0 + 1;
        }
        if self.cursor.0 > self.stop.0 {
            None
        } else {
            Some(self.cursor.clone())
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Coords4(i32, i32, i32, i32);

impl Coords for Coords4 {
    fn offset(&self, d: i32) -> Self {
        Self(self.0 + d, self.1 + d, self.2 + d, self.3 + d)
    }

    fn upto(&self, other: &Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(Upto4 {
            start: self.clone(),
            stop: other.clone(),
            cursor: Self(self.0, self.1, self.2, self.3 - 1),
        })
    }
}

struct Upto4 {
    start: Coords4,
    stop: Coords4,
    cursor: Coords4,
}

impl Iterator for Upto4 {
    type Item = Coords4;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.3 = self.cursor.3 + 1;
        if self.cursor.3 > self.stop.3 {
            self.cursor.3 = self.start.3;
            self.cursor.2 = self.cursor.2 + 1;
        }
        if self.cursor.2 > self.stop.2 {
            self.cursor.2 = self.start.2;
            self.cursor.1 = self.cursor.1 + 1;
        }
        if self.cursor.1 > self.stop.1 {
            self.cursor.1 = self.start.1;
            self.cursor.0 = self.cursor.0 + 1;
        }
        if self.cursor.0 > self.stop.0 {
            None
        } else {
            Some(self.cursor.clone())
        }
    }
}

type Grid<C> = HashMap<C, bool>;
type Game<C> = (C, C, Grid<C>);

pub fn run() {
    let mut grid3 = parse_grid();
    let mut grid4 = three2four(&grid3);
    for i in (0..6) {
        println!("---STEP3---");
        grid3 = step(grid3);
        println!("---STEP4---");
        grid4 = step(grid4);
    }
    println!("part 1: {}", score(&grid3)); // expect 322
    println!("part 2: {}", score(&grid4)); // expect 2000
}

fn score<T>(game: &Game<T>) -> usize {
    let (_, _, grid) = game;
    grid.iter().fold(0, |n, (_, v)| n + if *v { 1 } else { 0 })
}

fn step<C: Coords + Eq + Hash + Copy + std::fmt::Debug>(grid: Game<C>) -> Game<C> {
    let (min, max, grid) = grid;
    let min = min.offset(-1);
    let max = max.offset(1);
    let mut newgrid = Grid::new();
    for c in min.upto(&max) {
        let v = gol(&grid, &c);
        newgrid.insert(c.clone(), v);
    }
    (min, max, newgrid)
}

fn gol<C: Coords + Eq + Hash + std::fmt::Debug>(grid: &Grid<C>, c: &C) -> bool {
    let an = active_neighbors(grid, c);
    match grid.get(c) {
        Some(&true) => {
            if an == 2 || an == 3 {
                true
            } else {
                false
            }
        }
        _ => {
            if an == 3 {
                true
            } else {
                false
            }
        }
    }
}

fn active_neighbors<C: Coords + Eq + Hash + std::fmt::Debug>(grid: &Grid<C>, c: &C) -> usize {
    c.neighbors().filter(|x| x != c).fold(0, |res, dc| {
        res + match grid.get(&dc) {
            Some(&true) => 1,
            _ => 0,
        }
    })
}

fn parse_grid() -> Game<Coords3> {
    let mut grid = Grid::new();
    let mut row = 0;
    let mut maxcol = 0;
    for s in common::parse_lines::<String>() {
        let mut col = 0;
        for ch in s.trim().chars() {
            let c = Coords3(row, col, 0);
            let v = match ch {
                '#' => true,
                '.' => false,
                _ => panic!(">>> {:?}", c),
            };
            grid.insert(c, v);
            col = col + 1;
        }
        maxcol = if col > maxcol { col - 1 } else { maxcol };
        row = row + 1;
    }
    (Coords3(0, 0, 0), Coords3(row - 1, maxcol, 0), grid)
}

fn three2four(game3: &Game<Coords3>) -> Game<Coords4> {
    let (min, max, grid) = game3;
    (min.to4(), max.to4(), to4(grid))
}

fn to4(grid3: &Grid<Coords3>) -> Grid<Coords4> {
    let mut res = Grid::new();
    for (k, v) in grid3 {
        res.insert(k.to4(), *v);
    }
    res
}
