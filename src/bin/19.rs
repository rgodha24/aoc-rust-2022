use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub};

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_to_blueprints(input, 24);
    let answer = solve_blueprints(blueprints);
    let answer = answer
        .iter()
        .enumerate()
        .map(|(idx, geode)| (idx.clone() as u32 + 1) * (*geode as u32))
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut blueprints = parse_to_blueprints(input, 32);
    blueprints.truncate(3);

    let answer = solve_blueprints(blueprints);

    Some(answer.into_iter().product())
}

fn solve_blueprints(blueprints: Vec<Blueprint>) -> Vec<u32> {
    let answer: Vec<_> = blueprints
        .par_iter()
        .progress_count(blueprints.len() as u64)
        .map(|b| solve_blueprint(b.clone()).into())
        .collect();

    answer
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn solve_blueprint(blueprint: Blueprint) -> u8 {
    let max_robots = blueprint.max_robots();
    let costs: Costs = blueprint.clone().into();
    let first_state: State = blueprint.into();
    let mut heap: BinaryHeap<_> = vec![first_state].into();
    let mut set: HashSet<State> = HashSet::new();
    let mut max_geode = u8::MIN;

    while let Some(state) = heap.pop() {
        // we've already calculated this branch
        if set.contains(&state) {
            continue;
        }
        match tick_state(&state, &max_robots, &costs) {
            TickReturn::Loop(answers) => {
                heap.extend(answers);
            }
            TickReturn::Answer(answer) => {
                if answer > max_geode {
                    max_geode = answer;
                }
            }
        }
        set.insert(state);
    }

    max_geode
}

fn tick_state(state: &State, max_robots: &MaxRobots, costs: &Costs) -> TickReturn {
    // if this is the last cycle, building robots won't make a difference, so just return early
    if state.time_remaining == 1 {
        let mut new = state.clone();
        new.tick();
        return TickReturn::Answer(new.inventory[Geode]);
    }

    let mut answers = Vec::new();

    if state.inventory > costs[Geode] {
        answers.push(state.w_new_robot(costs, Geode));

        // if we can build a geode, we always should
        return TickReturn::Loop(answers);
    }
    if state.inventory > costs[Ore] && state.robots[Ore] < max_robots[Ore] {
        answers.push(state.w_new_robot(costs, Ore));
    }
    if state.inventory > costs[Clay] && state.robots[Clay] < max_robots[Clay] {
        answers.push(state.w_new_robot(costs, Clay));
    }
    if state.inventory > costs[Obsidian] && state.robots[Obsidian] < max_robots[Obsidian] {
        answers.push(state.w_new_robot(costs, Obsidian));
    }

    // no change
    let mut new = state.clone();
    new.tick();
    answers.push(new);

    TickReturn::Loop(answers)
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

#[derive(Debug, Clone)]
enum TickReturn {
    Loop(Vec<State>),
    Answer(u8),
}

#[derive(Debug, Clone, Copy)]
enum Robots {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
use Robots::*;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Resources(u8, u8, u8, u8);

struct Costs([Resources; 4]);

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    inventory: Resources,
    robots: Resources,
    time_remaining: u8,
}

#[derive(Default, Debug, Hash, PartialEq, Eq)]
struct MaxRobots(u8, u8, u8);

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl From<Blueprint> for Costs {
    fn from(blueprint: Blueprint) -> Self {
        Costs([
            blueprint.ore_cost.into(),
            blueprint.clay_cost.into(),
            blueprint.obsidian_cost.into(),
            blueprint.geode_cost.into(),
        ])
    }
}

impl From<Cost> for Resources {
    fn from(cost: Cost) -> Self {
        Resources(
            cost.ore as u8,
            cost.clay as u8,
            cost.obsidian as u8,
            cost.geode as u8,
        )
    }
}

impl From<Blueprint> for State {
    fn from(blueprint: Blueprint) -> Self {
        State {
            inventory: Default::default(),
            robots: Resources(blueprint.ore_r_count as u8, 0, 0, 0),
            time_remaining: blueprint.time_remaining as u8,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl State {
    fn tick(&mut self) {
        self.time_remaining -= 1;
        self.inventory += self.robots;
    }
    fn priority(&self) -> isize {
        // this is so ugly LMAO
        let ans = self.inventory.geode() * 100
            + self.robots.geode() * 500
            + self.inventory.obsidian() * 10
            + self.robots.obsidian() * 50;

        (ans as isize) - self.time_remaining as isize * 15 - self.inventory.clay() as isize * 2
    }

    fn w_new_robot(&self, costs: &Costs, robot: Robots) -> State {
        let mut state = self.clone();
        state.inventory = state.inventory - costs[robot].clone();
        state.tick();
        state.robots.inc(robot, 1);

        state
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, other: Resources) -> Self::Output {
        Resources(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, other: Resources) -> Self::Output {
        Resources(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, other: Resources) {
        *self = *self + other;
    }
}

impl Index<Robots> for Resources {
    type Output = u8;
    fn index(&self, index: Robots) -> &Self::Output {
        match index {
            Robots::Ore => &self.0,
            Robots::Clay => &self.1,
            Robots::Obsidian => &self.2,
            Robots::Geode => &self.3,
        }
    }
}

impl IndexMut<Robots> for Resources {
    fn index_mut(&mut self, index: Robots) -> &mut Self::Output {
        match index {
            Robots::Ore => &mut self.0,
            Robots::Clay => &mut self.1,
            Robots::Obsidian => &mut self.2,
            Robots::Geode => &mut self.3,
        }
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Resources {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            return Ordering::Less;
        }
        if self.1 < other.1 {
            return Ordering::Less;
        }
        if self.2 < other.2 {
            return Ordering::Less;
        }
        if self.3 < other.3 {
            return Ordering::Less;
        }

        Ordering::Greater
    }
}

impl Resources {
    fn ore(&self) -> usize {
        self.0 as usize
    }
    fn clay(&self) -> usize {
        self.1 as usize
    }
    fn obsidian(&self) -> usize {
        self.2 as usize
    }
    fn geode(&self) -> usize {
        self.3 as usize
    }
    fn inc(&mut self, robot: Robots, amount: u8) {
        self[robot] += amount;
    }
}

impl Index<Robots> for Costs {
    type Output = Resources;
    fn index(&self, index: Robots) -> &Self::Output {
        match index {
            Robots::Ore => &self.0[0],
            Robots::Clay => &self.0[1],
            Robots::Obsidian => &self.0[2],
            Robots::Geode => &self.0[3],
        }
    }
}

impl Index<Robots> for MaxRobots {
    type Output = u8;
    fn index(&self, index: Robots) -> &Self::Output {
        match index {
            Robots::Ore => &self.0,
            Robots::Clay => &self.1,
            Robots::Obsidian => &self.2,
            Robots::Geode => &u8::MAX,
        }
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd)]
struct Blueprint {
    ore_r_count: u32,
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
            self.max_ore_robots() as u8,
            self.max_clay_robots() as u8,
            self.max_obsidian_robots() as u8,
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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(3472));
    }
}
