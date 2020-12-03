use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    part2: bool,
    input: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Output {
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
    let area = solve_part1(area);
    Ok(Output {
        collisions: count_collisions(&area),
        rendered: render(&area),
    })
}

fn solve_part1(mut area: Area) -> Area {
    let (mut x, mut y) = (3, 1);
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
        x = x + 3;
        y = y + 1;
    }
    area
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

    fn part1(input: &'static str) -> Input {
        Input {
            part2: false,
            input: input.to_string(),
        }
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

        let output = solve(part1(input)).unwrap();

        assert_eq!(expected_output.to_string(), output.rendered);
        assert_eq!(7, output.collisions);
    }
}
