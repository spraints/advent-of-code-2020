use super::common;
use std::io::Read;
use std::str::FromStr;

pub fn run<R: Read>(r: R) {
    let room = common::parse_lines(r);
    println!(
        "part 1: {}",
        count_occupied(run_until_not_changed(room.clone(), &NeighborCounter {}, 4))
    );
    println!(
        "part 2: {}",
        count_occupied(run_until_not_changed(room, &VisibleCounter {}, 5))
    );
}

fn run_until_not_changed<C: Counter>(mut room: Vec<Row>, counter: &C, max_vis: usize) -> Vec<Row> {
    loop {
        match update_spaces(&room, counter, max_vis) {
            None => return room,
            Some(newroom) => room = newroom,
        };
    }
}

fn update_spaces<C: Counter>(room: &[Row], counter: &C, max_vis: usize) -> Option<Vec<Row>> {
    let mut newroom = Vec::new();
    let mut flipped = false;
    for (row, row_row) in room.iter().enumerate() {
        let mut spaces = Vec::new();
        for (col, space) in row_row.spaces.iter().enumerate() {
            match space {
                Space::Floor => spaces.push(Space::Floor),
                Space::Empty => {
                    if counter.count(room, row, col) == 0 {
                        flipped = true;
                        spaces.push(Space::Occupied);
                    } else {
                        spaces.push(Space::Empty);
                    }
                }
                Space::Occupied => {
                    if counter.count(room, row, col) >= max_vis {
                        flipped = true;
                        spaces.push(Space::Empty);
                    } else {
                        spaces.push(Space::Occupied);
                    }
                }
            };
        }
        newroom.push(Row { spaces });
    }
    if flipped {
        Some(newroom)
    } else {
        None
    }
}

const DIR: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

trait Counter {
    fn count(&self, room: &[Row], row: usize, col: usize) -> usize;
}

struct VisibleCounter {}

impl Counter for VisibleCounter {
    fn count(&self, room: &[Row], row: usize, col: usize) -> usize {
        let mut res = 0;
        for (dr, dc) in DIR.iter() {
            let mut n = 1;
            loop {
                match get_space(room, (row as i32) + n * dr, (col as i32) + n * dc) {
                    Some(Space::Floor) => {
                        n += 1;
                        continue;
                    }
                    Some(Space::Occupied) => res += 1,
                    _ => (),
                };
                break;
            }
        }
        res
    }
}

fn get_space(room: &[Row], row: i32, col: i32) -> Option<Space> {
    if row < 0 || col < 0 {
        return None;
    }
    room.get(row as usize)
        .and_then(|row| row.spaces.get(col as usize).copied())
}

struct NeighborCounter {}

impl Counter for NeighborCounter {
    fn count(&self, room: &[Row], row: usize, col: usize) -> usize {
        let mut res = 0;
        for (dr, dc) in DIR.iter() {
            if let Some(Space::Occupied) = get_space(room, (row as i32) + dr, (col as i32) + dc) {
                res += 1;
            }
        }
        res
    }
}

fn count_occupied(room: Vec<Row>) -> usize {
    room.iter().map(count_occupied_row).sum()
}

fn count_occupied_row(row: &Row) -> usize {
    row.spaces
        .iter()
        .filter(|space| is_occupied_space(*space))
        .count()
}

fn is_occupied_space(space: &Space) -> bool {
    matches!(space, Space::Occupied)
}

#[derive(Clone, Copy, Debug)]
enum Space {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone)]
struct Row {
    spaces: Vec<Space>,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spaces = s
            .trim()
            .chars()
            .map(|c| match c {
                '.' => Space::Floor,
                'L' => Space::Empty,
                _ => panic!("unexpected {:?}", c),
            })
            .collect();
        Ok(Self { spaces })
    }
}
