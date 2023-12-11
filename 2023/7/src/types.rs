// Types and impls for both parts
use std::{collections::BTreeMap, str::FromStr};

use itertools::Itertools;
use lib::impl_parse;

#[derive(Debug, Clone)]
pub struct Input {
    pub hands: Vec<Hand>,
}

impl_parse! {
    Token {
        (Cards Number)+
    }
    without r"[\s]+"
    as {
        Cards Cards "[2-9AKQJT]{5}"
        Number usize "[0-9]+"
    }
}

impl FromStr for Input {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let hands = Token::parse(input)?;
        Ok(Self {
            hands: hands
                .iter()
                .map(|(cards, bid)| Hand {
                    cards: BTreeMap::from_iter(
                        get_cards(cards).unwrap().into_iter().counts_by(|card| card),
                    ),
                    bid: get_number(bid).unwrap_or_default(),
                })
                .collect_vec(),
        })
    }
}

fn get_number(token: &Token) -> Option<usize> {
    match token {
        Token::Number(val) => Some(*val),
        _ => None,
    }
}

fn get_cards(token: &Token) -> Option<Vec<Card>> {
    match token {
        Token::Cards(val) => Some(val.0.to_owned()),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: BTreeMap<Card, usize>,
    pub bid: usize,
}

impl Hand {
    pub fn rank(&self) -> Rank {
        let mut values = self.cards.values().sorted().rev().map(|c| c.to_owned());
        let first = values.next().unwrap_or_default();
        let second = values.next().unwrap_or_default();
        match (first, second) {
            (5, _) => Rank::FiveOfAKind,
            (4, _) => Rank::FourOfAKind,
            (3, 2) => Rank::FullHouse,
            (3, 1) => Rank::ThreeOfAKind,
            (2, 2) => Rank::TwoPair,
            (2, 1) => Rank::OnePair,
            (1, 1) => Rank::HighCard,
            _ => Rank::NotAMaster,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    NotAMaster = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub enum Card {
    A = 13,
    K = 12,
    Q = 11,
    J = 10,
    T = 9,
    N9 = 8,
    N8 = 7,
    N7 = 6,
    N6 = 5,
    N5 = 4,
    N4 = 3,
    N3 = 2,
    #[default]
    N2 = 1,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "A" => Self::A,
            "K" => Self::K,
            "Q" => Self::Q,
            "J" => Self::J,
            "T" => Self::T,
            "9" => Self::N9,
            "8" => Self::N8,
            "7" => Self::N7,
            "6" => Self::N6,
            "5" => Self::N5,
            "4" => Self::N4,
            "3" => Self::N3,
            "2" => Self::N2,
            _ => Err("Wrong card!")?,
        })
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Cards(Vec<Card>);

impl FromStr for Cards {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| c.to_string().parse().unwrap_or_default())
                .collect_vec(),
        ))
    }
}
