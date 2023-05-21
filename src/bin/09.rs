pub fn part_one(input: &str) -> Option<usize> {
    let directions = parse(input);
    let mut set: HashSet<Point> = HashSet::new();
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);

    for d in directions {
        head.move_direction(d);

        tail.follow(head);

        set.insert(tail);
    }

    Some(set.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let directions = parse(input);
    let mut set: HashSet<Point> = HashSet::new();
    let mut knots: [Point; 10] = (0..10)
        .map(|_| Point(0, 0))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    for d in directions {
        knots[0].move_direction(d);
        let mut last = knots[0];
        for i in 1..10 {
            knots[i].follow(last);
            last = knots[i];
        }

        set.insert(knots[9]);
    }

    Some(set.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(isize, isize);

impl Point {
    // cant call it 'move' :(
    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Up => self.1 += 1,
            Down => self.1 -= 1,
            Left => self.0 -= 1,
            Right => self.0 += 1,
        }
    }
    fn follow(&mut self, point: Point) {
        let distance_x = (point.0 - self.0).abs();
        let distance_y = (point.1 - self.1).abs();
        let distance = distance_x + distance_y;

        let is_connected = distance <= 1 || (distance == 2 && distance_x == distance_y);
        if !is_connected {
            let x = point.0 - self.0;
            let y = point.1 - self.1;

            self.0 += x.signum();
            self.1 += y.signum();
        }
    }
}

fn parse(input: &str) -> Vec<Direction> {
    let mut ans = Vec::new();
    for l in input.lines() {
        let parts: Vec<_> = l.split(" ").collect();
        let dir = Direction::from(parts[0]);
        let amount = parts[1].parse::<usize>().unwrap();
        for _ in 0..amount {
            ans.push(dir);
        }
    }

    ans
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use std::collections::HashSet;

use Direction::*;

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        match input {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            _ => panic!("Invalid direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
