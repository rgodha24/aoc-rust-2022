use indicatif::ProgressBar;

pub fn part_one(input: &str) -> Option<isize> {
    let mut nums = parse(input, 1);

    mix(&mut nums, 1);

    Some(gps(&nums))
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut nums = parse(input, 811589153);
    mix(&mut nums, 10);

    Some(gps(&nums))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, 20, part_one, input);
    advent_of_code::solve!(2, 20, part_two, input);
}

#[derive(Clone, Copy)]
struct Num {
    /// the value of the actual number when parsed
    value: isize,
    /// the index this was when parsed
    index: usize,
}

fn mix(nums: &mut Vec<Num>, rounds: usize) {
    let bar = ProgressBar::new(rounds as u64 * nums.len() as u64);
    for _ in 0..rounds {
        for i in 0..nums.len() {
            let index = nums.iter().position(|n| n.index == i).unwrap();
            let num = nums.remove(index);
            let mut new_index = index as isize + num.value;
            if new_index < 0 {
                new_index += (nums.len() * (new_index.abs() as usize / nums.len() + 1)) as isize;
            }

            new_index %= nums.len() as isize;
            nums.insert(new_index as usize, num);

            bar.inc(1);

            // print_nums(&nums);
        }
    }
}

fn gps(nums: &Vec<Num>) -> isize {
    let zero_val = nums.iter().position(|n| n.value == 0).unwrap();

    (1000..=3000)
        .filter(|i| i % 1000 == 0)
        .map(|i| nums[(i + zero_val) % nums.len()].value)
        .sum()
}

fn print_nums(nums: &Vec<Num>) {
    for num in nums {
        print!("{},", num.value);
    }
    println!();
}

fn parse(input: &str, key: isize) -> Vec<Num> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| Num {
            value: line.parse::<isize>().unwrap() * key,
            index,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
