use advent_of_code::helpers::{Grid, Point};
use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

pub fn part_one(input: &str) -> Option<usize> {
    let mut board = parse(input);

    let goal = Point {
        x: board.at(0).width() as isize - 1,
        y: board.at(0).height() as isize - 1,
    };
    let start = Location {
        p: Point::from_x_y((0, 0)),
        time: 1,
    };

    Some(pathfind_to(&mut board, start, goal) + 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut board = parse(input);

    let end = Point {
        x: board.at(0).width() as isize - 1,
        y: board.at(0).height() as isize - 1,
    };
    let start = Point::from_x_y((0, 0));

    // plus 2 because move to end spot, then move back one
    let time = pathfind_to(&mut board, Location { p: start, time: 1 }, end) + 2;
    // plus 2 because move to end spot, then move back one
    let time = pathfind_to(&mut board, Location { p: end, time }, start) + 2;
    // just plus one because we only have to move to the end spot
    let time = pathfind_to(&mut board, Location { p: start, time }, end) + 1;

    Some(time)
}

fn pathfind_to(board: &mut Board, start: Location, goal: Point) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    // we can stay in the starting spot for a long time. this simulates that bc i forgot that was
    // an option lmao
    for t in 0..25 {
        let time = t + start.time;
        let _ = board.at(time);

        heap.push(Location { p: start.p, time })
    }

    while let Some(Location { p, time }) = heap.pop() {
        if p == goal {
            return time;
        }

        let g = board.at(time + 1);

        let mut neighbors = g.neighbors_of(p);
        neighbors.push(p);
        neighbors.retain(|p| g[p.clone()] == Tile(vec![]) && seen.insert((p.clone(), time + 1)));

        for p in neighbors {
            heap.push(Location { p, time: time + 1 })
        }
    }

    panic!("no solution found")
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);

    advent_of_code::solve!(1, 24, part_one, input);
    advent_of_code::solve!(2, 24, part_two, input);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Location {
    p: Point,
    time: usize,
}

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Grid<Tile>>,
}

#[derive(Debug, Clone, PartialEq)]
struct Tile(Vec<Direction>);

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Board {
    let grid = input
        .lines()
        .map(|s| &s[1..(s.len() - 1)])
        .filter_map(|l| {
            if l.contains('#') {
                None
            } else {
                Some(l.chars().map(|c| c.into()).collect::<Vec<_>>())
            }
        })
        .collect::<Vec<_>>();

    Board {
        grid: vec![Grid(grid)],
    }
}

impl Board {
    fn at(&mut self, time: usize) -> &'_ Grid<Tile> {
        if time < self.grid.len() {
            &self.grid[time]
        } else {
            let old = self.grid.get(time - 1).expect("time before is available");
            let new = iter_grid(old);

            self.grid.push(new);

            &self.grid[time]
        }
    }
}

fn iter_grid(old: &Grid<Tile>) -> Grid<Tile> {
    let mut new = old.clone();

    for row in new.iter_mut() {
        for c in row.iter_mut() {
            c.0.clear()
        }
    }
    let cols = old.len() as isize;
    let rows = old[0].len() as isize;

    for (y, row) in old.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let x = x as isize;
            let y = y as isize;

            for dir in &tile.0 {
                let mut point = Point::from_x_y(match dir {
                    Direction::Up => (x, y - 1),
                    Direction::Down => (x, y + 1),
                    Direction::Left => (x - 1, y),
                    Direction::Right => (x + 1, y),
                });

                point.x = point.x.rem_euclid(rows);
                point.y = point.y.rem_euclid(cols);

                new[point].0.push(dir.clone());
            }
        }
    }

    new
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        Tile(match c {
            '#' => panic!("wall in tile"),
            '.' => vec![],
            t => vec![t.into()],
        })
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            d => panic!("invalid direction {d}"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir: char = self.clone().into();
        write!(f, "{}", dir)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = &self.0;
        let tile = if d.len() == 1 {
            let dir: char = d[0].clone().into();
            dir.to_string()
        } else if d.len() == 0 {
            ".".to_string()
        } else if d.len() < 10 {
            d.len().to_string()
        } else {
            "+".to_string()
        };

        write!(f, "{}", tile)
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            write!(f, "{}", row)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }

    #[test]
    fn test_iter_grid() {
        let mut grid = parse(
            r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#,
        );

        let min1 = r#".>3.<.
<..<<.
>2.22.
>v..^<
"#;

        let min2 = r#".2>2..
.^22^<
.>2.^>
.>..<.
"#;

        assert_eq!(grid.at(1).to_string(), min1);
        assert_eq!(grid.at(2).to_string(), min2);
    }
}
