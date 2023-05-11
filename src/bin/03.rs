use std::collections::HashSet;

const words: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut answer = vec![];
    for line in lines {
        let first_half = line.split_at(line.len() / 2).0;
        let second_half = line.split_at(line.len() / 2).1;
        let first_half: HashSet<char> = first_half.chars().into_iter().collect();

        for c in second_half.chars() {
            if first_half.contains(&c) {
                answer.push(c);
                break;
            }
        }
    }

    // a-z 1-26, A-Z 27-52
    let mut sum = 0;
    for c in answer {
        sum += words.find(c).unwrap() as u32 + 1;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().into_iter().collect::<Vec<_>>();
    let mut answer = vec![];
    for i in 0..(lines.len()) {
        if i % 3 != 0 {
            continue;
        }
        let first: HashSet<char> = lines[i].chars().into_iter().collect();
        let second: HashSet<char> = lines[i + 1].chars().into_iter().collect();
        let third: Vec<char> = lines[i + 2].chars().into_iter().collect();

        for c in third {
            if first.contains(&c) && second.contains(&c) {
                answer.push(c);
                break;
            }
        }
    }

    // a-z 1-26, A-Z 27-52
    let mut sum = 0;
    for c in answer {
        sum += words.find(c).unwrap() as u32 + 1;
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
