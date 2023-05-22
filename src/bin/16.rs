use regex::Regex;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Index, IndexMut},
};

pub fn part_one(input: &str) -> Option<u32> {
    let (all, map) = parse(input);
    let start: State = all.into();

    let mut queue: BinaryHeap<_> = vec![start].into();
    let mut max_pressure = 0;
    let mut set: HashSet<State> = HashSet::new();

    let distances = distances(&map);

    while let Some(state) = queue.pop() {
        if !set.insert(state.clone()) {
            continue;
        }

        let moves = state.moves(&map, &distances);
        // no moves that make sense, so we prune branch + check max ticks
        if moves.len() == 0 {
            let mut new_state = state.clone();
            new_state.tick(&map, 30 - new_state.minute);
            if new_state.pressure > max_pressure {
                max_pressure = new_state.pressure;
            }
            continue;
        }

        let new_states = moves
            .into_iter()
            .map(|m| calculate_move(m, &map))
            .filter_map(|m| {
                return match m {
                    MoveResult::More(state) => Some(state),
                    MoveResult::Done(pressure) => {
                        max_pressure = max_pressure.max(pressure);
                        None
                    }
                };
            });

        queue.extend(new_states);
    }

    Some(max_pressure as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, 16, part_one, input);
    advent_of_code::solve!(2, 16, part_two, input);
}

fn parse(input: &str) -> (Vec<Valve>, Map) {
    let mut valves = Vec::new();
    let mut map = HashMap::new();
    let mut names: HashMap<&str, Name> = vec![("AA", Name(0))].into_iter().collect();

    // this next 10 lines of code made me want to die..
    let re = Regex::new("[A-Z]{2}").unwrap();
    let mut captures = re
        .find_iter(input)
        .map(|i| i.as_str())
        .collect::<HashSet<_>>();
    captures.remove("AA");
    let mut index = 0;
    for name in captures {
        index += 1;
        names.insert(name, Name(index));
    }

    for line in input.lines() {
        let line = line
            .to_string()
            .replace("tunnel ", "tunnels ")
            .replace("leads ", "lead ")
            .replace("valve ", "valves ");
        let valve = string_to_valve(line, &names);
        valves.push(valve.clone());
        map.insert(valve.name.clone().into(), valve);
    }

    (valves, map)
}

fn calculate_move(mut value: Move, map: &Map) -> MoveResult {
    let time_remaining = 30 - value.start_state.minute;
    // if we can't make it to our goal in time, just tick and return
    if time_remaining <= value.distance {
        value.start_state.tick(map, time_remaining);
        return MoveResult::Done(value.start_state.pressure);
    }

    let mut state = value.start_state;
    state.move_to(value.end);
    state.tick(map, value.distance + 1);
    state.open_valve(value.end);

    MoveResult::More(state)
}

// Floyd Warshell algo
fn distances(map: &Map) -> Distances {
    let num = map.len();
    let mut valve_distances = vec![vec![u8::MAX / 2; num]; num];

    for (name, valve) in map {
        for tunnel in valve.paths.iter() {
            valve_distances[*name][*tunnel] = 1;
            valve_distances[*tunnel][*name] = 1;
        }
    }

    for i in 0..num {
        valve_distances[i][i] = 0;
    }

    for k in 0..num {
        for i in 0..num {
            for j in 0..num {
                valve_distances[i][j] =
                    valve_distances[i][j].min(valve_distances[i][k] + valve_distances[k][j]);
            }
        }
    }

    valve_distances
}

type Distances = Vec<Vec<u8>>;
type Map = HashMap<Name, Valve>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Move {
    distance: u8,
    end: Name,
    start_state: State,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    current: Name,
    open: Vec<Name>,
    closed: Vec<Name>,
    minute: u8,
    pressure: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: Name,
    paths: Vec<Name>,
    flow_rate: u8,
}

#[derive(Debug, Clone)]
enum MoveResult {
    More(State),
    Done(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
struct Name(u8);

impl State {
    fn tick(&mut self, map: &Map, times: u8) {
        self.minute += times;
        self.pressure += self
            .open
            .iter()
            .map(|n| map.get(n).unwrap())
            .map(|v| v.flow_rate)
            .sum::<u8>() as usize
            * times as usize;
    }

    fn move_to(&mut self, to: Name) {
        self.current = to;
    }

    fn open_valve(&mut self, name: Name) {
        self.open.push(name);
        self.closed.retain(|v| v != &name);
    }

    fn moves(&self, map: &Map, distances: &Distances) -> Vec<Move> {
        self.closed
            .iter()
            .map(|n| map.get(n).unwrap())
            .filter(|v| v.flow_rate > 0)
            .map(|v| {
                let distance = distances[self.current][v.name];

                Move {
                    distance,
                    end: v.name.clone(),
                    start_state: self.clone(),
                }
            })
            .collect()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.minute.cmp(&self.minute)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.minute.cmp(&other.minute))
    }
}

impl From<Vec<Valve>> for State {
    fn from(valves: Vec<Valve>) -> Self {
        Self {
            current: Name(0),
            closed: valves.iter().map(|v| v.name.clone()).collect(),
            open: Vec::new(),
            minute: 0,
            pressure: 0,
        }
    }
}
impl<T> Index<Name> for Vec<T> {
    type Output = T;

    fn index(&self, index: Name) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl<T> IndexMut<Name> for Vec<T> {
    fn index_mut(&mut self, index: Name) -> &mut T {
        &mut self[index.0 as usize]
    }
}

fn string_to_valve(value: String, names: &HashMap<&str, Name>) -> Valve {
    let (name, flow_rate, paths) = sscanf::sscanf!(
        value,
        "Valve {} has flow rate={}; tunnels lead to valves {}",
        str,
        u8,
        String
    )
    .expect("valid input");

    let paths = paths
        .split(",")
        .map(|s| *names.get(s.trim()).unwrap())
        .collect();

    Valve {
        name: names[name],
        flow_rate,
        paths,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
