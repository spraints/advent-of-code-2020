use super::common;
use std::collections::HashMap;

type Coords = (i32, i32, i32, i32);
type Grid = HashMap<Coords, bool>;
type Game = (Coords, Coords, Grid);

pub fn run() {
    let mut grid = parse_grid();
    pg(&grid);
    for i in (0..6) {
        println!("---STEP---");
        grid = step(grid);
        pg(&grid);
    }
    let active = grid
        .2
        .iter()
        .fold(0, |n, (_, v)| n + if *v { 1 } else { 0 });
    // 560 is too high.
    println!("part 1: {}", active);
}

fn pg(grid: &Game) {
    let (min, max, grid) = grid;
    for z in (min.2..=max.2) {
        for w in (min.3..=max.3) {
            println!("z = {}, w = {}", z, w);
            for row in (min.0..=max.0) {
                for col in (min.1..=max.1) {
                    match grid.get(&(row, col, z, w)) {
                        Some(&true) => print!("#"),
                        _ => print!("."),
                    }
                }
                println!("");
            }
        }
    }
}

fn step(grid: Game) -> Game {
    let (min, max, grid) = grid;
    let min = add(&min, (-1, -1, -1, -1));
    let max = add(&max, (1, 1, 1, 1));
    let mut newgrid = Grid::new();
    for z in (min.2..=max.2) {
        for w in (min.3..=max.3) {
            for row in (min.0..=max.0) {
                for col in (min.1..=max.1) {
                    let c = (row, col, z, w);
                    let v = gol(&grid, &c);
                    if v {
                        println!("ACTIVE: {:?}", c);
                    }
                    newgrid.insert(c, v);
                }
            }
        }
    }
    (min, max, newgrid)
}

fn gol(grid: &Grid, c: &Coords) -> bool {
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

fn active_neighbors(grid: &Grid, c: &Coords) -> usize {
    let mut res = 0;
    for dx in (-1..=1) {
        for dy in (-1..=1) {
            for dz in (-1..=1) {
                for dw in (-1..=1) {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        let dc = add(c, (dx, dy, dz, dw));
                        match grid.get(&dc) {
                            Some(&true) => res = res + 1,
                            _ => (),
                        };
                    }
                }
            }
        }
    }
    //println!("{:?} => {}", c, res);
    res
}

fn add(c1: &Coords, c2: Coords) -> Coords {
    (c1.0 + c2.0, c1.1 + c2.1, c1.2 + c2.2, c1.3 + c2.3)
}

fn parse_grid() -> Game {
    let mut grid = HashMap::new();
    let mut row = 0;
    let mut maxcol = 0;
    for s in common::parse_lines::<String>() {
        let mut col = 0;
        for ch in s.trim().chars() {
            let c = (row, col, 0, 0);
            let v = match ch {
                '#' => true,
                '.' => false,
                _ => panic!(">>> {:?}", c),
            };
            //if v {
            //    println!("ACTIVE: {:?}", c);
            //}
            grid.insert(c, v);
            col = col + 1;
        }
        maxcol = if col > maxcol { col - 1 } else { maxcol };
        row = row + 1;
    }
    ((0, 0, 0, 0), (row - 1, maxcol, 0, 0), grid)
}
