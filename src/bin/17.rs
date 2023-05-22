use indicatif::{ProgressBar, ProgressIterator};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn part_one(input: &str) -> Option<isize> {
    Some(solve(input, 2022))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(solve(input, 1000000000000))
}

fn solve(input: &str, years: usize) -> isize {
    let rocks: Vec<Vec<Point>> = vec![
        vec![[2, 0], [3, 0], [4, 0], [5, 0]],
        vec![[2, 1], [3, 1], [3, 2], [3, 0], [4, 1]],
        vec![[2, 0], [3, 0], [4, 0], [4, 1], [4, 2]],
        vec![[2, 0], [2, 1], [2, 2], [2, 3]],
        vec![[2, 0], [3, 0], [2, 1], [3, 1]],
    ]
    .iter()
    .map(|r| r.iter().map(|p| (p[0], p[1])).collect::<Vec<_>>())
    .collect();

    // map of Cycle to (year, ymax)
    let mut cycle_cache: HashMap<Cycle, (usize, isize)> = HashMap::new();

    let jets: Vec<Jet> = input.trim().chars().map(|c| Jet::from(c)).collect();

    let mut board = Board::default();
    let mut i = 0;
    let mut cycle_found = false;
    let mut cycled_y = 0;

    let mut year = 0;
    while year < years {
        let mut shape = Shape::new(rocks[year % 5].clone(), board.get_max_y() + 4);

        loop {
            let jet = jets[i % jets.len()];
            shape.jet(&board, jet);
            i += 1;
            if shape.drop(&board) == DropResult::Done {
                shape.settle(&mut board);
                break;
            }
        }

        year += 1;

        if !cycle_found {
            let cycle = Cycle {
                jet_index: (i % jets.len()) as u8,
                shape_index: (year % 5) as u8,
                y_offsets: board.get_y_offsets(),
            };
            if let Some((cycle_year, cycle_y)) = cycle_cache.get(&cycle) {
                cycle_found = true;
                let cycle_length = year - cycle_year;
                let y_per_cycle = board.get_max_y() - cycle_y;
                let cycles = (years - year) / cycle_length;
                cycled_y = cycles as isize * y_per_cycle + 1;
                let extra_years = cycles as usize * cycle_length;
                year += extra_years;
            } else {
                cycle_cache.insert(cycle, (year, board.get_max_y()));
            }
        }
    }

    board.get_max_y() + cycled_y
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Jet {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Board {
    board: HashSet<Point>,
    max_y: isize,
}

#[derive(Debug, Clone)]
struct Shape {
    points: Vec<Point>,
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cycle {
    jet_index: u8,
    shape_index: u8,
    /// the distance between y_max and the highest point in the column
    y_offsets: [u8; 7],
}

impl Shape {
    fn new(points: Vec<Point>, y: isize) -> Self {
        Self { points, x: 0, y }
    }
    fn jet(&mut self, board: &Board, jet: Jet) {
        let mut new_points = Vec::new();
        let x = match jet {
            Jet::Left => self.x - 1,
            Jet::Right => self.x + 1,
        };
        for p in &self.points {
            let new_p = (p.0 + x, p.1 + self.y);
            if board.board.contains(&new_p) || new_p.0 < 0 || new_p.0 > 6 {
                return;
            }
            new_points.push(new_p);
        }
        self.x = x;
    }
    fn drop(&mut self, board: &Board) -> DropResult {
        let mut new_points = Vec::new();
        let y = self.y - 1;
        for p in &self.points {
            let new_p = (p.0 + self.x, p.1 + y);
            if board.board.contains(&new_p) || new_p.1 < 0 {
                return DropResult::Done;
            }
            new_points.push(new_p);
        }
        self.y -= 1;

        DropResult::Continue
    }
    fn settle(&self, board: &mut Board) {
        for (x, y) in &self.points {
            let p = (*x + self.x, *y + self.y);
            board.insert(p);
        }
    }
}

impl Board {
    fn get_max_y(&self) -> isize {
        self.max_y
    }

    fn insert(&mut self, point: Point) {
        self.max_y = self.max_y.max(point.1);
        self.board.insert(point);
    }

    fn get_y_offsets(&self) -> [u8; 7] {
        let mut y_offsets = [0; 7];
        for x in 0..7 {
            for y in (0..=self.max_y).rev() {
                if self.board.contains(&(x, y)) {
                    y_offsets[x as usize] = (self.max_y - y) as u8;
                    break;
                }
            }
        }
        y_offsets
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: HashSet::new(),
            max_y: -1,
        }
    }
}

impl From<char> for Jet {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid jet"),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in (0..(self.max_y + 2)).rev() {
            s.push('|');
            for x in 0..=6 {
                if self.board.contains(&(x, y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('|');
            s.push('\n');
        }
        s.push_str("+-------+");

        write!(f, "{}", s)
    }
}

type Point = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DropResult {
    Done,
    Continue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
