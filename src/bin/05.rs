pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);

    for Move { from, to, amount } in moves {
        let moved = (0..amount)
            .map(|_| stacks[from].pop().unwrap())
            .collect::<Vec<_>>();

        stacks[to].extend(moved);
    }

    Some(stacks.iter().filter_map(|s| s.last()).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);

    for Move { from, to, amount } in moves {
        let mut moved = (0..amount)
            .map(|_| stacks[from].pop().unwrap())
            .collect::<Vec<_>>();
        // way too lazy to modify the top part and do it right so just reverse it :)
        moved.reverse();

        stacks[to].extend(moved);
    }

    Some(stacks.iter().filter_map(|s| s.last()).collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, 5, part_one, input);
    advent_of_code::solve!(2, 5, part_two, input);
}

type Stack = Vec<char>;

#[derive(Debug, Clone)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

// that was painful
fn parse(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let mut stacks: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|l| {
            l.replace("   ", "0")
                .replace("[", "")
                .replace("]", "")
                .replace(" ", "")
        })
        .map(|l| l.chars().collect())
        .collect();

    stacks.remove(stacks.len() - 1);

    let moves = parts[1].lines().map(|l| l.into()).collect();

    let mut stacks_new: Vec<Stack> = vec![Vec::new(); stacks[0].len()];

    let min = stacks.iter().map(|s| s.len()).min().unwrap();

    for i in 0..min {
        for j in stacks.iter().rev().into_iter() {
            let char = j[i];
            if char == '0' {
                continue;
            }
            stacks_new[i].push(char);
        }
    }

    (stacks_new, moves)
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let (amount, from, to) =
            sscanf::scanf!(value, "move {} from {} to {}", usize, usize, usize).unwrap();

        Self {
            from: from - 1,
            to: to - 1,
            amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
