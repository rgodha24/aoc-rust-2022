pub fn part_one(input: &str) -> Option<u32> {
    let cals = input
        .split("\n\n")
        .map(|group| group.split("\n").map(|f| str::parse::<u32>(f).unwrap()))
        .map(|group| group.sum());

    cals.max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let cals = input
        .split("\n\n")
        .map(|group| group.split("\n").map(|f| str::parse::<u32>(f).unwrap()))
        .map(|group| group.sum());

    let mut test: Vec<u32> = cals.collect();

    test.sort();
    test.reverse();

    Some(test[0] + test[1] + test[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input).unwrap(), 24000);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input).unwrap(), 45000);
    }
}
