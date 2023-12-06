use std::{cell::RefCell, collections::HashSet, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: HashSet<u32>,
    pub card_numbers: HashSet<u32>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !input.starts_with("Card") {
            return Err("Wrong data format!")?;
        }

        let mut data = input.split(':');
        let id: u32 = data
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let mut sets = data.next().unwrap().split('|');
        let (winning_numbers, card_numbers) = (
            HashSet::from_iter(
                sets.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect_vec(),
            ),
            HashSet::from_iter(
                sets.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect_vec(),
            ),
        );

        Ok(Self {
            id,
            winning_numbers,
            card_numbers,
        })
    }
}

#[derive(Debug)]
pub struct CardInfo {
    pub points: usize,
    pub count: RefCell<u32>,
}
