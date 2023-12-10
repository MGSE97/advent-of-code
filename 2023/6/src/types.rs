// Types and impls for both parts
use std::str::FromStr;

use itertools::Itertools;
use lib::impl_parse;

#[derive(Debug, Clone)]
pub struct Input {
    pub times: Vec<u32>,
    pub distances: Vec<u32>,
}

impl_parse! {
    Token {
        Time:     Number+
        Distance: Number+
    }
    without r"[:\s]+"
    as {
        Time     "Time"
        Distance "Distance"
        Number u32 "[0-9]+"
    }
}

impl FromStr for Input {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, times, _, distances) = Token::parse(input)?;
        Ok(Self {
            times: get_numbers(times),
            distances: get_numbers(distances),
        })
    }
}

fn get_numbers(arr: Vec<Token>) -> Vec<u32> {
    arr.into_iter()
        .filter_map(|time| match time {
            Token::Number(val) => Some(val),
            _ => None,
        })
        .collect_vec()
}
