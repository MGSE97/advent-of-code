use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
pub struct Input {
    pub seeds: Vec<u64>,
    pub maps: HashMap<MapKey, Map>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MapKey {
    pub from: String,
    pub to: String,
}
impl MapKey {
    pub(crate) fn new(from: &str, to: &str) -> MapKey {
        MapKey {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub ranges: Vec<MapRange>,
}

#[derive(Debug)]
pub struct MapRange {
    pub lower: u64,
    pub upper: u64,
    pub target_lower: u64,
    pub target_upper: u64,
}

impl Map {
    pub fn map(&self, id: u64) -> u64 {
        match self
            .ranges
            .iter()
            .find(|range| range.lower <= id && id < range.upper)
        {
            Some(range) => range.target_lower + id - range.lower,
            None => id,
        }
    }
    pub fn rmap(&self, id: u64) -> u64 {
        match self
            .ranges
            .iter()
            .find(|range| range.target_lower <= id && id < range.target_upper)
        {
            Some(range) => range.lower + id - range.target_lower,
            None => id,
        }
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut sections = input.lines();
        let seeds = sections
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split_whitespace()
            .map(|id| id.parse().unwrap())
            .collect_vec();

        let maps = HashMap::from_iter(sections.skip(1).join("\n").split("\n\n").map(|section| {
            let mut parts = section.split(':');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        }));
        Ok(Self { seeds, maps })
    }
}

impl FromStr for MapKey {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(['-', ' ']);
        Ok(Self {
            from: parts.next().unwrap().to_string(),
            to: parts.nth(1).unwrap().to_string(),
        })
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut ranges = Vec::new();
        for line in input.lines() {
            let parts: Vec<u64> = line
                .split_whitespace()
                .filter_map(|p| u64::from_str(p).ok())
                .collect_vec();

            if parts.len() == 3 {
                ranges.push(MapRange {
                    lower: parts[1],
                    upper: parts[1] + parts[2],
                    target_lower: parts[0],
                    target_upper: parts[0] + parts[2],
                });
            }
        }
        Ok(Self { ranges })
    }
}
