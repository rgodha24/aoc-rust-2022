use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i64> {
    let monkeys = parse(input);
    let root = monkeys.get("root").expect("root monkey");

    Some(root.calc(&monkeys))
}

pub fn part_two(input: &str) -> Option<i64> {
    // plan of attack: find which side of a calc depends on humn. we then know what the other side has
    // to be, and can pass that value into the function recursively

    let map = parse(input);
    let root = map.get("root").expect("root monkey");
    let (_, left_name, right_name) = match root {
        Monkey::Number(_) => panic!("root is a number"),
        Monkey::Calculated(op, left, right) => (op, left, right),
    };

    let left = map.get(left_name).unwrap();
    let right = map.get(right_name).unwrap();

    let (goal, monkey) = if left.depends_on_humn(&map) {
        (right.calc(&map), left_name)
    } else {
        (left.calc(&map), right_name)
    };

    let humn_value = calc_humn(goal, monkey, &map);

    Some(humn_value)
}

fn calc_humn(value: i64, name: &str, map: &HashMap<&str, Monkey>) -> i64 {
    // if we have found humn, we return the value that we expect humn to be
    if name == "humn" {
        return value;
    }

    let monkey = map.get(name).unwrap();
    match monkey {
        Monkey::Number(number) => *number,
        Monkey::Calculated(op, ls, rs) => {
            let left = map.get(ls).unwrap();
            let right = map.get(rs).unwrap();

            let (new_name, new_goal) = if left.depends_on_humn(map) || ls == &"humn" {
                let right_val = right.calc(map);
                // now we switch the operator. we know what we want this to equal, and the right
                // hand side, so with that we can figure out what the left side of the monkey has
                // to equal
                let new_goal = match op {
                    Operator::Add => value - right_val,
                    Operator::Multiply => value / right_val,
                    Operator::Divide => value * right_val,
                    Operator::Subtract => value + right_val,
                };
                (ls, new_goal)
            } else {
                let left_val = left.calc(map);
                let new_goal = match op {
                    Operator::Add => value - left_val,
                    Operator::Multiply => value / left_val,
                    Operator::Divide => value * left_val,
                    Operator::Subtract => left_val - value,
                };
                (rs, new_goal)
            };

            calc_humn(new_goal, new_name, map)
        }
    }
}

#[derive(Debug)]
enum Monkey<'a> {
    Number(i64),
    Calculated(Operator, &'a str, &'a str),
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, 21, part_one, input);
    advent_of_code::solve!(2, 21, part_two, input);
}

impl Monkey<'_> {
    fn calc(&self, map: &HashMap<&str, Monkey>) -> i64 {
        let ans = match self {
            Monkey::Number(number) => *number,
            Monkey::Calculated(operator, left, right) => {
                let left = map.get(left).unwrap().calc(map);
                let right = map.get(right).unwrap().calc(map);
                match operator {
                    Operator::Add => left + right,
                    Operator::Multiply => left * right,
                    Operator::Divide => left / right,
                    Operator::Subtract => left - right,
                }
            }
        };

        ans
    }

    fn depends_on_humn(&self, map: &HashMap<&str, Monkey>) -> bool {
        match self {
            // in a perfect world you could check if this number monkey is humn, but my data
            // modelling was bad so it's not possible...
            Monkey::Number(_) => false,
            Monkey::Calculated(_, left, right) => {
                if left == &"humn" || right == &"humn" {
                    true
                } else {
                    let left = map.get(left).unwrap();
                    let right = map.get(right).unwrap();
                    left.depends_on_humn(map) || right.depends_on_humn(map)
                }
            }
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Monkey> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (monkey, name) = parse_one(line);
        map.insert(name, monkey);
    }

    map
}

fn parse_one(input: &str) -> (Monkey, &str) {
    let parts = input.split(": ").collect::<Vec<_>>();
    let name = parts[0];
    match parts[1].parse::<i64>() {
        Ok(number) => (Monkey::Number(number), name),
        Err(_) => {
            let parts: Vec<_> = parts[1].split(" ").collect();
            let left = parts[0];
            let operator = parts[1];
            let right = parts[2];
            (Monkey::Calculated(operator.into(), left, right), name)
        }
    }
}

impl From<&str> for Operator {
    fn from(input: &str) -> Self {
        match input {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            "-" => Operator::Subtract,
            _ => panic!("Unknown operator: {}", input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
