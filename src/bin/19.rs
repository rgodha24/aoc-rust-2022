pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_to_blueprints(input);
    let mut sum = 0;
    for blueprint in blueprints {
        sum += calc_geodes(blueprint, 24);
        println!("one done ");
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn calc_geodes(mut blueprint: Blueprint, iterations: usize) -> u32 {
    blueprint.tick();
    if iterations == 1 {
        return blueprint.inventory.geode;
    }

    let max_ore_cost = blueprint
        .ore_cost
        .ore
        .max(blueprint.obsidian_cost.ore)
        .max(blueprint.clay_cost.ore)
        .max(blueprint.geode_cost.ore);

    let mut answers = Vec::new();
    answers.push(calc_geodes(blueprint.with_no_change(), iterations - 1));
    let mut branches = 1;
    if blueprint.inventory >= blueprint.ore_cost && max_ore_cost > blueprint.ore_r_count {
        answers.push(calc_geodes(blueprint.with_new_ore_robot(), iterations - 1));
        branches += 1;
    }
    if blueprint.inventory >= blueprint.clay_cost {
        answers.push(calc_geodes(blueprint.with_new_clay_robot(), iterations - 1));
        branches += 1;
    }
    if blueprint.inventory >= blueprint.obsidian_cost {
        answers.push(calc_geodes(
            blueprint.with_new_obsidian_robot(),
            iterations - 1,
        ));
        branches += 1;
    }
    if blueprint.inventory >= blueprint.geode_cost {
        answers.push(calc_geodes(
            blueprint.with_new_geode_robot(),
            iterations - 1,
        ));
        branches += 1;
    }

    print!("{} + ", branches);

    *answers.iter().max().unwrap_or(&0)
}

fn parse_to_blueprints(input: &str) -> Vec<Blueprint> {
    let blueprints: Vec<Vec<&str>> = input
        .split("\n")
        .map(|i| i.split_whitespace().collect())
        .collect();

    let blueprints: Vec<&Vec<_>> = blueprints.iter().filter(|i| i.len() > 0).collect();

    println!("{:?}", blueprints);

    let blueprints: Vec<Blueprint> = blueprints
        .iter()
        .map(|b| Blueprint {
            ore_r_count: 1,
            ore_cost: Cost {
                ore: b[6].parse().unwrap(),
                ..Default::default()
            },
            clay_cost: Cost {
                ore: b[12].parse().unwrap(),
                ..Default::default()
            },
            obsidian_cost: Cost {
                ore: b[18].parse().unwrap(),
                clay: b[21].parse().unwrap(),
                ..Default::default()
            },
            geode_cost: Cost {
                ore: b[27].parse().unwrap(),
                obsidian: b[30].parse().unwrap(),
                ..Default::default()
            },

            ..Default::default()
        })
        .collect();

    blueprints
}

#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    ore_r_count: u32,
    clay_r_count: u32,
    obsidian_r_count: u32,
    geode_r_count: u32,
    inventory: Cost,
    ore_cost: Cost,
    obsidian_cost: Cost,
    clay_cost: Cost,
    geode_cost: Cost,
}

impl Blueprint {
    pub fn tick(&mut self) {
        self.inventory.ore += self.ore_r_count;
        self.inventory.clay += self.clay_r_count;
        self.inventory.obsidian += self.obsidian_r_count;
        self.inventory.geode += self.geode_r_count;
    }

    pub fn with_new_ore_robot(&self) -> Self {
        let mut new = self.clone();
        new.ore_r_count += 1;

        new
    }
    pub fn with_new_clay_robot(&self) -> Self {
        let mut new = self.clone();
        new.clay_r_count += 1;

        new
    }
    pub fn with_new_obsidian_robot(&self) -> Self {
        let mut new = self.clone();
        new.obsidian_r_count += 1;

        new
    }
    pub fn with_new_geode_robot(&self) -> Self {
        let mut new = self.clone();
        new.geode_r_count += 1;

        new
    }
    pub fn with_no_change(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(9));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_ord() {
        assert!(
            Cost {
                ore: 4,
                clay: 4,
                ..Default::default()
            } > Cost {
                ore: 3,
                clay: 4,
                ..Default::default()
            }
        )
    }
}
