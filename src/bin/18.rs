use std::collections::{BinaryHeap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let points = parse(input);

    let mut sum = 0;
    for (x, y, z) in &points {
        for (dx, dy, dz) in &OFFSETS {
            if !points.contains(&(x + dx, y + dy, z + dz)) {
                sum += 1;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // start from min x,y,z (bc we know that's outside air). create a stack. check offsets for lava. if lava, add, otherwise add to stack.
    let lava = parse(input);
    let x = lava.iter().map(|(x, _, _)| x);
    let y = lava.iter().map(|(_, y, _)| y);
    let z = lava.iter().map(|(_, _, z)| z);
    let minx = x.clone().min().unwrap() - 1;
    let maxx = x.max().unwrap() + 1;
    let miny = y.clone().min().unwrap() - 1;
    let maxy = y.max().unwrap() + 1;
    let minz = z.clone().min().unwrap() - 1;
    let maxz = z.max().unwrap() + 1;

    let mut heap: BinaryHeap<_> = vec![(minx, miny, minz)].into();
    let mut visited = HashSet::new();

    let mut sum = 0;

    while let Some((x, y, z)) = heap.pop() {
        if !visited.insert((x, y, z)) {
            continue;
        }

        // out of bounds
        if x < minx || x > maxx || y < miny || y > maxy || z < minz || z > maxz {
            continue;
        }

        for (dx, dy, dz) in &OFFSETS {
            let point = (x + dx, y + dy, z + dz);
            if lava.contains(&point) {
                sum += 1;
            } else {
                heap.push(point);
            }
        }
    }

    Some(sum)
}
const OFFSETS: [Point; 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
];

type Point = (i8, i8, i8);

fn parse(input: &str) -> HashSet<Point> {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();

            (nums[0], nums[1], nums[2])
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, 18, part_one, input);
    advent_of_code::solve!(2, 18, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
