use super::common;
use std::collections::HashMap;
use std::hash::Hash;

trait Coords {
    fn offset(&self, d: i32) -> Self;
    fn start_cursor(&self) -> Self;
    fn step(&self, start: &Self, stop: &Self) -> Option<Self>
    where
        Self: Sized;

    fn upto(&self, other: &Self) -> Upto<Self>
    where
        Self: Sized + Copy,
    {
        Upto {
            start: *self,
            stop: *other,
            cursor: self.start_cursor(),
        }
    }

    fn neighbors(&self) -> Upto<Self>
    where
        Self: Sized + Copy,
    {
        let start = self.offset(-1);
        let stop = self.offset(1);
        start.upto(&stop)
    }
}

struct Upto<C: Sized> {
    start: C,
    stop: C,
    cursor: C,
}

impl<C: Coords + Copy> Iterator for Upto<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor.step(&self.start, &self.stop) {
            None => None,
            Some(newcur) => {
                self.cursor = newcur;
                Some(self.cursor)
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Coords3(i32, i32, i32);

impl Coords for Coords3 {
    fn offset(&self, d: i32) -> Self {
        Self(self.0 + d, self.1 + d, self.2 + d)
    }

    fn start_cursor(&self) -> Self {
        Self(self.0, self.1, self.2 - 1)
    }

    fn step(&self, start: &Self, stop: &Self) -> Option<Self> {
        let mut newcur = *self;
        newcur.2 = newcur.2 + 1;
        if newcur.2 > stop.2 {
            newcur.2 = start.2;
            newcur.1 = newcur.1 + 1;
        }
        if newcur.1 > stop.1 {
            newcur.1 = start.1;
            newcur.0 = newcur.0 + 1;
        }
        if newcur.0 > stop.0 {
            None
        } else {
            Some(newcur)
        }
    }
}

impl Coords3 {
    fn to4(&self) -> Coords4 {
        Coords4(self.0, self.1, self.2, 0)
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Coords4(i32, i32, i32, i32);

impl Coords for Coords4 {
    fn offset(&self, d: i32) -> Self {
        Self(self.0 + d, self.1 + d, self.2 + d, self.3 + d)
    }

    fn start_cursor(&self) -> Self {
        Self(self.0, self.1, self.2, self.3 - 1)
    }

    fn step(&self, start: &Self, stop: &Self) -> Option<Self> {
        let mut newcur = *self;
        newcur.3 = newcur.3 + 1;
        if newcur.3 > stop.3 {
            newcur.3 = start.3;
            newcur.2 = newcur.2 + 1;
        }
        if newcur.2 > stop.2 {
            newcur.2 = start.2;
            newcur.1 = newcur.1 + 1;
        }
        if newcur.1 > stop.1 {
            newcur.1 = start.1;
            newcur.0 = newcur.0 + 1;
        }
        if newcur.0 > stop.0 {
            None
        } else {
            Some(newcur)
        }
    }
}

type Grid<C> = HashMap<C, bool>;
type Game<C> = (C, C, Grid<C>);

pub fn run() {
    let mut grid3 = parse_grid();
    let mut grid4 = three2four(&grid3);
    for _ in 0..6 {
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

fn step<C: Coords + Eq + Hash + Copy + std::fmt::Debug + std::marker::Copy>(
    grid: Game<C>,
) -> Game<C> {
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

fn gol<C: Coords + Eq + Hash + std::fmt::Debug + std::marker::Copy>(grid: &Grid<C>, c: &C) -> bool {
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

fn active_neighbors<C: Coords + Eq + Hash + std::fmt::Debug + std::marker::Copy>(
    grid: &Grid<C>,
    c: &C,
) -> usize {
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
