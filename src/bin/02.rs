pub fn part_one(input: &str) -> Option<u32> {
    let vals = input
        .split("\n")
        .map(|val| val.split(" ").collect::<Vec<_>>());

    let answer = vals.map(|round| {
        match [round[0], round[1]] {
            // win
            ["A", "Y"] => 6 + 2,
            ["B", "Z"] => 6 + 3,
            ["C", "X"] => 6 + 1,
            ["A", "X"] => 3 + 1,
            ["B", "Y"] => 3 + 2,
            ["C", "Z"] => 3 + 3,
            ["A", "Z"] => 0 + 3,
            ["B", "X"] => 0 + 1,
            ["C", "Y"] => 0 + 2,
            _ => panic!("Invalid input"),
        }
    });

    return Some(answer.sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let vals = input
        .split("\n")
        .map(|val| val.split(" ").collect::<Vec<_>>());

    let answer = vals.map(|round| {
        match [round[0], round[1]] {
            // lose
            ["A", "X"] => 0 + 3,
            ["B", "X"] => 0 + 1,
            ["C", "X"] => 0 + 2,

            // draw
            ["A", "Y"] => 3 + 1,
            ["B", "Y"] => 3 + 2,
            ["C", "Y"] => 3 + 3,

            // win
            ["A", "Z"] => 6 + 2,
            ["B", "Z"] => 6 + 3,
            ["C", "Z"] => 6 + 1,
            _ => panic!("Invalid input"),
        }
    });

    return Some(answer.sum());
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input).unwrap(), 15);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input).unwrap(), 12);
    }
}
