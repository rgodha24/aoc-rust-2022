use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Benchmark(BTreeMap<u8, Part>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
struct Part {
    part_1: String,
    part_2: String,
}

impl Benchmark {
    pub fn from_file() -> Self {
        let bench = std::fs::read_to_string("benchmark.json").unwrap();
        serde_json::from_str(&bench).unwrap()
    }
    pub fn add(&mut self, day: u8, p: u8, duration: Duration) {
        let mut part = self.0.entry(day).or_default().clone();
        let duration = format!("{:.2?}", duration);
        match p {
            1 => part.part_1 = duration,
            2 => part.part_2 = duration,
            p => panic!("invalid part {}", p),
        };

        self.0.insert(day, part);
    }
    pub fn write(self) {
        let mut keys = self.0.keys().map(|k| *k).collect::<Vec<_>>();
        keys.sort();
        let mut ordered_map = BTreeMap::new();
        for key in keys {
            ordered_map.insert(key, self.0.get(&key).unwrap());
        }
        let stringified = serde_json::to_string_pretty(&ordered_map).unwrap();
        std::fs::write("benchmark.json", stringified).unwrap();
    }
}
