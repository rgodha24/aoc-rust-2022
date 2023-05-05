use cached::proc_macro::cached;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Sub;

pub fn part_one(input: &str) -> Option<u32> {
    const MINUTES: u32 = 24;
    let blueprints = parse_to_blueprints(input, MINUTES);
    println!("blueprints: {:?}", blueprints.len());

    let answer: Vec<_> = blueprints
        .par_iter()
        .progress_count(blueprints.len() as u64)
        .map(|b| solve_blueprint(b.clone()))
        .enumerate()
        .collect();
    let answer = answer
        .iter()
        .map(|(idx, geode)| (*idx as u32 + 1) * geode)
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn solve_blueprint(blueprint: Blueprint) -> u32 {
    let max_robots = blueprint.max_robots();
    let mut heap: BinaryHeap<_> = vec![blueprint].into_iter().collect();
    let mut max_geode = u32::MIN;
    let mut iterations: u64 = 0;

    while let Some(blueprint) = heap.pop() {
        let answers = tick_blueprint(blueprint, max_robots);

        match answers {
            BlueprintReturn::Blueprint(answers) => {
                //bar.inc_length(answers.len() as u64);
                heap.extend(answers);
            }
            BlueprintReturn::Answer(answer) => {
                if answer > max_geode {
                    max_geode = answer;
                    // bar.println(format!("new max: {:?}", max_geode));
                }
            }
            BlueprintReturn::DeadBranch => {}
        }
        iterations += 1;
        // bar.inc(1);

        if iterations > 200_000_000 {
            break;
        }
    }

    // bar.finish();

    return max_geode;
}

#[derive(Debug, Clone)]
enum BlueprintReturn {
    Blueprint(Vec<Blueprint>),
    Answer(u32),
    DeadBranch,
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct MaxRobots(u32, u32, u32);

// #[cached]
fn tick_blueprint(mut blueprint: Blueprint, max_robots: MaxRobots) -> BlueprintReturn {
    if blueprint.time_remaining == 0 {
        return BlueprintReturn::Answer(blueprint.inventory.geode);
    }

    blueprint.tick();

    let mut answers = Vec::new();

    if blueprint.inventory > blueprint.geode_cost {
        answers.push(blueprint.with_new_geode_robot());

        // if we can build a geode, we always should
        return BlueprintReturn::Blueprint(answers);
    }
    if blueprint.inventory > blueprint.ore_cost && blueprint.ore_r_count < max_robots.0 {
        answers.push(blueprint.with_new_ore_robot());
    }
    if blueprint.inventory > blueprint.clay_cost && blueprint.clay_r_count < max_robots.1 {
        answers.push(blueprint.with_new_clay_robot());
    }
    if blueprint.inventory > blueprint.obsidian_cost && blueprint.obsidian_r_count < max_robots.2 {
        answers.push(blueprint.with_new_obsidian_robot());
    }

    answers.push(blueprint.with_no_change());
    BlueprintReturn::Blueprint(answers)
}

fn parse_to_blueprints(input: &str, time_remaining: u32) -> Vec<Blueprint> {
    let blueprints: Vec<Vec<&str>> = input
        .split("\n")
        .map(|i| i.split_whitespace().collect())
        .collect();

    let blueprints: Vec<&Vec<_>> = blueprints.iter().filter(|i| i.len() > 0).collect();

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
            time_remaining,
            ..Default::default()
        })
        .collect();

    blueprints
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd)]
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
    time_remaining: u32,
    new_robots: [u32; 4],
}

impl Sub for Cost {
    type Output = Cost;

    fn sub(self, other: Cost) -> Cost {
        Cost {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Ord for Blueprint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ore < other.ore {
            return Ordering::Less;
        }
        if self.clay < other.clay {
            return Ordering::Less;
        }
        if self.obsidian < other.obsidian {
            return Ordering::Less;
        }
        if self.geode < other.geode {
            return Ordering::Less;
        }

        Ordering::Greater
    }
}

impl Blueprint {
    pub fn priority(&self) -> u32 {
        self.inventory.geode * 100
            + self.geode_r_count * 500
            + self.inventory.obsidian * 10
            + self.obsidian_r_count * 50
            - self.time_remaining * 15
            - self.inventory.clay * 2
    }
    fn to_arr(&self) -> [u8; 8] {
        [
            self.ore_r_count as u8,
            self.clay_r_count as u8,
            self.obsidian_r_count as u8,
            self.geode_r_count as u8,
            self.inventory.ore as u8,
            self.inventory.clay as u8,
            self.inventory.obsidian as u8,
            self.inventory.geode as u8,
        ]
    }

    pub fn tick(&mut self) {
        self.inventory.ore += self.ore_r_count - self.new_robots[0];
        self.inventory.clay += self.clay_r_count - self.new_robots[1];
        self.inventory.obsidian += self.obsidian_r_count - self.new_robots[2];
        self.inventory.geode += self.geode_r_count - self.new_robots[3];
    }

    pub fn with_new_ore_robot(&self) -> Self {
        let mut new = self.clone();
        new.ore_r_count += 1;
        new.inventory = self.inventory - self.ore_cost;
        new.time_remaining -= 1;
        new.new_robots = [1, 0, 0, 0];

        new
    }
    pub fn with_new_clay_robot(&self) -> Self {
        let mut new = self.clone();
        new.clay_r_count += 1;
        new.inventory = self.inventory - self.clay_cost;
        new.time_remaining -= 1;
        new.new_robots = [0, 1, 0, 0];

        new
    }
    pub fn with_new_obsidian_robot(&self) -> Self {
        let mut new = self.clone();
        new.obsidian_r_count += 1;
        new.inventory = self.inventory - self.obsidian_cost;
        new.time_remaining -= 1;
        new.new_robots = [0, 0, 1, 0];

        new
    }
    pub fn with_new_geode_robot(&self) -> Self {
        let mut new = self.clone();
        new.geode_r_count += 1;
        new.inventory = self.inventory - self.geode_cost;
        new.time_remaining -= 1;
        new.new_robots = [0, 0, 0, 1];

        new
    }
    pub fn with_no_change(&self) -> Self {
        let mut new = self.clone();
        new.time_remaining -= 1;
        new.new_robots = [0, 0, 0, 0];

        new
    }

    fn max_ore_robots(&self) -> u32 {
        self.geode_cost.ore.max(self.clay_cost.ore)
    }
    fn max_clay_robots(&self) -> u32 {
        self.geode_cost.clay.max(self.obsidian_cost.clay)
    }
    fn max_obsidian_robots(&self) -> u32 {
        self.geode_cost.obsidian
    }

    fn max_robots(&self) -> MaxRobots {
        MaxRobots(
            self.max_ore_robots(),
            self.max_clay_robots(),
            self.max_obsidian_robots(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_one_real() {
        let input = advent_of_code::read_file("inputs", 19);
        assert_eq!(part_one(&input), Some(1834));
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
        );

        assert!(
            Cost {
                ore: 4,
                ..Default::default()
            } >= Cost {
                ore: 4,
                ..Default::default()
            }
        );

        assert!(
            Cost {
                ore: 5,
                clay: 10,
                ..Default::default()
            } < Cost {
                obsidian: 13,
                ..Default::default()
            }
        )
    }
}
