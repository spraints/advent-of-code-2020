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
}

type Path = Vec<(usize, usize)>;

type Line = (Vec<Space>, bool);
type Area = Vec<Line>;

pub fn solve(input: Input) -> Result<Output, String> {
    let area = parse(&input.input)?;
    let slopes = if input.part2 {
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    } else {
        vec![(3, 1)]
    };
    let runs: Vec<RunResult> = slopes
        .into_iter()
        .map(|slope| do_run(&area, slope))
        .collect();
    let product = runs.iter().fold(1, |prod, res| prod * res.collisions);
    Ok(Output { product, runs })
}

fn do_run(area: &Area, slope: (usize, usize)) -> RunResult {
    let (mut x, mut y) = slope;
    let mut path = Path::new();
    let mut collisions = 0;
    while y < area.len() {
        if let Some((line, _)) = area.get(y) {
            let len = line.len();
            if let Some(space) = line.get(x % len) {
                if let Space::Tree = space {
                    collisions += 1;
                }
                path.push((x % len, y));
            }
        }
        x = x + slope.0;
        y = y + slope.1;
    }
    RunResult {
        slope,
        collisions,
        rendered: render(&area, &path),
    }
}

fn render(area: &Area, path: &Path) -> String {
    let mut res = String::with_capacity(area.len() * (area[0].0.len() + 7));
    for (y, line) in area.iter().enumerate() {
        let (spaces, has_arrow) = line;
        for (x, space) in spaces.iter().enumerate() {
            res.push(if path_includes(path, &(x, y)) {
                match space {
                    Space::Empty => 'O',
                    Space::Tree => 'X',
                }
            } else {
                match space {
                    Space::Empty => '.',
                    Space::Tree => '#',
                }
            });
        }
        if *has_arrow {
            res.push_str("  --->");
        }
        res.push('\n');
    }
    res
}

fn path_includes(path: &Path, coords: &(usize, usize)) -> bool {
    for step in path {
        if step == coords {
            return true;
        }
    }
    return false;
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

    #[test]
    fn test_part2() {
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

        let output = solve(mk_input(true, input)).unwrap();

        assert_eq!(336, output.product);
    }
}
