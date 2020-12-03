use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    part2: bool,
    input: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub struct Output {
    product: i64,
    runs: Vec<RunResult>,
}

#[derive(Serialize)]
pub struct RunResult {
    slope: (usize, usize),
    collisions: i64,
    rendered: String,
}

enum Space {
    Empty,
    Tree,
    Path,
    Collision(i64),
}

type Line = (Vec<Space>, bool);
type Area = Vec<Line>;

pub fn solve(input: Input) -> Result<Output, String> {
    let area = parse(&input.input)?;
    let result = do_run(area, (3, 1));
    Ok(Output {
        product: result.collisions,
        runs: vec![result],
    })
}

fn do_run(mut area: Area, slope: (usize, usize)) -> RunResult {
    let (mut x, mut y) = slope;
    while y < area.len() {
        if let Some((line, _)) = area.get_mut(y) {
            let len = line.len();
            if let Some(space) = (*line).get_mut(x % len) {
                *space = match *space {
                    Space::Empty => Space::Path,
                    Space::Tree => Space::Collision(1),
                    Space::Path => Space::Path,
                    Space::Collision(n) => Space::Collision(n + 1),
                }
            }
        }
        x = x + slope.0;
        y = y + slope.1;
    }
    RunResult {
        slope: (3, 1),
        collisions: count_collisions(&area),
        rendered: render(&area),
    }
}

fn count_collisions(area: &Area) -> i64 {
    let mut count = 0;
    for (line, _) in area {
        for space in line {
            if let Space::Collision(n) = space {
                count += n;
            }
        }
    }
    count
}

fn render(area: &Area) -> String {
    let mut res = String::with_capacity(area.len() * (area[0].0.len() + 7));
    for line in area {
        let (spaces, has_arrow) = line;
        for space in spaces {
            res.push(match space {
                Space::Empty => '.',
                Space::Tree => '#',
                Space::Path => 'O',
                Space::Collision(_) => 'X',
            });
        }
        if *has_arrow {
            res.push_str("  --->");
        }
        res.push('\n');
    }
    res
}

fn parse(s: &String) -> Result<Area, String> {
    Ok(s.lines().map(|line| parse_line(line)).collect())
}

fn parse_line<'a>(line: &'a str) -> Line {
    let arrow = line.contains("-->");
    let line = line
        .chars()
        .filter_map(|c| match c {
            '.' => Some(Space::Empty),
            '#' => Some(Space::Tree),
            _ => None,
        })
        .collect();
    (line, arrow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn mk_input(part2: bool, input: &'static str) -> Input {
        let input = input.to_string();
        Input { part2, input }
    }

    #[test]
    fn test_part1() {
        let input = "\
            ..##.........##.........##.........##.........##.........##.......  --->\n\
            #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..\n\
            .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.\n\
            ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#\n\
            .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.\n\
            ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->\n\
            .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#\n\
            .#........#.#........#.#........#.#........#.#........#.#........#\n\
            #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...\n\
            #...##....##...##....##...##....##...##....##...##....##...##....#\n\
            .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->\n\
            ";
        let expected_output = "\
            ..##.........##.........##.........##.........##.........##.......  --->\n\
            #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..\n\
            .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.\n\
            ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#\n\
            .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.\n\
            ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->\n\
            .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#\n\
            .#........#.#........X.#........#.#........#.#........#.#........#\n\
            #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...\n\
            #...##....##...##....##...#X....##...##....##...##....##...##....#\n\
            .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->\n\
            ";

        let output = solve(mk_input(false, input)).unwrap();

        assert_eq!(expected_output.to_string(), output.runs[0].rendered);
        assert_eq!(7, output.product);
    }
    }
}
