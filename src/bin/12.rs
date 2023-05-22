use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    ops::Index,
};

use rayon::prelude::*;

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input.to_string());
    let start = map.get_start();
    let end = map.get_end();

    pathfind(map, start, end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from(input.to_string());
    let mut starts = vec![map.get_start()];
    let end = map.get_end();

    for (i, row) in map.locations.iter().enumerate() {
        for (j, loc) in row.iter().enumerate() {
            if *loc == Location(0) {
                starts.push((i, j));
            }
        }
    }

    // there's a much faster way to do this but rust is already fast + rayon parellelizes it
    // so i dont rly care enough to do it faster
    Some(
        starts
            .par_iter()
            .filter_map(|&start| pathfind(map.clone(), start, end))
            .min()
            .expect("min") as u32,
    )
}

fn pathfind(map: Map, start: Point, end: Point) -> Option<u32> {
    let mut heap: BinaryHeap<Node> = vec![Node {
        cost: 0,
        point: start,
    }]
    .into();
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some(Node { cost, point }) = heap.pop() {
        if point == end {
            return Some(cost);
        }

        let candidates = map.valid_moves(point);
        for candidate in candidates {
            if visited.insert(candidate) {
                heap.push(Node {
                    cost: cost + 1,
                    point: candidate,
                });
            }
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, 12, part_one, input);
    advent_of_code::solve!(2, 12, part_two, input);
}

#[derive(Debug, Clone)]
struct Map {
    locations: Vec<Vec<Location>>,
    rows: usize,
    cols: usize,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: u32,
    point: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Location(u8);

type Point = (usize, usize);

const START: Location = Location(26);
const END: Location = Location(27);

impl Map {
    fn neighbors(&self, point: Point) -> Vec<Point> {
        let mut moves = Vec::new();
        let (row, col) = point;
        if row > 0 {
            moves.push((row - 1, col));
        }
        if row < self.rows - 1 {
            moves.push((row + 1, col));
        }
        if col > 0 {
            moves.push((row, col - 1));
        }
        if col < self.cols - 1 {
            moves.push((row, col + 1));
        }
        moves
    }

    fn valid_moves(&self, point: Point) -> Vec<Point> {
        let neighbors = self.neighbors(point);
        let current = self[point];
        neighbors
            .into_iter()
            .filter(|&p| current > self[p])
            .collect()
    }
    fn get_start(&self) -> Point {
        for (row, r) in self.locations.iter().enumerate() {
            for (col, c) in r.iter().enumerate() {
                if *c == START {
                    return (row, col);
                }
            }
        }
        panic!("No start found");
    }
    fn get_end(&self) -> Point {
        for (row, r) in self.locations.iter().enumerate() {
            for (col, c) in r.iter().enumerate() {
                if *c == END {
                    return (row, col);
                }
            }
        }
        panic!("No end found");
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // intentionally reversed
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    // less if self can't reach it, greater if self can reach it
    // this took WAY TOO LONG to write ig im just stupid
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Ordering::*;
        if self == &START {
            if other.0 < 2 {
                Greater
            } else {
                Less
            }
        } else if other == &START {
            Greater
        } else if self == &END {
            Greater
        } else if other == &END {
            if self.0 >= 24 {
                Greater
            } else {
                Less
            }
        } else if self.0 + 1 >= other.0 {
            Greater
        } else {
            Less
        }
    }
}

impl Index<Point> for Map {
    type Output = Location;

    fn index(&self, index: Point) -> &Self::Output {
        &self.locations[index.0][index.1]
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.locations {
            for point in row {
                write!(f, "{}", Into::<String>::into(*point))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<String> for Map {
    fn from(input: String) -> Self {
        let mut locations = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Location::from(c));
            }
            locations.push(row);
        }
        Map {
            rows: locations.len(),
            cols: locations[0].len(),
            locations,
        }
    }
}

impl Into<String> for Location {
    fn into(self) -> String {
        if self == START {
            "S".to_string()
        } else if self == END {
            "E".to_string()
        } else {
            String::from(('a' as u8 + self.0) as char)
        }
    }
}
impl From<char> for Location {
    fn from(s: char) -> Self {
        if s == 'S' {
            START
        } else if s == 'E' {
            END
        } else {
            Location(s as u8 - 'a' as u8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }

    #[test]
    fn test_point_ord() {
        assert!(Location::from('a') > Location::from('b'));
        assert!(START < Location::from('c'));
        assert!(Location::from('y') > END);
        assert!(Location::from('z') > Location::from('z'));
        assert!(Location::from('z') > Location::from('s'));
    }
}
