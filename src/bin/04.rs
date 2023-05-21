use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    Some(
        input
            .iter()
            .filter(|(a, b)| a.clone().all(|n| b.contains(&n)) || b.clone().all(|n| a.contains(&n)))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    Some(
        input
            .iter()
            .filter(|(a, b)| a.clone().any(|n| b.contains(&n)) || b.clone().any(|n| a.contains(&n)))
            .count(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn parse(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<usize> = l
                .split(",")
                .map(|p| p.split("-").map(|n| n.parse::<usize>().unwrap()))
                .flatten()
                .collect();
            (parts[0]..=parts[1], parts[2]..=parts[3])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
