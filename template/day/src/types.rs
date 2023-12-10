// Types and impls for both parts
use std::str::FromStr;

use itertools::Itertools;
use lib::impl_parse;

#[derive(Debug, Clone)]
pub struct Input {
    // ToDo: change this to fit your input
    pub numbers: Vec<u32>,
    pub strings: Vec<String>,
}

// ToDo: change this to fit your input
impl_parse! {
    Token {
        Numbers: Number+
        Strings: String+
    }
    without r"[:\s]+"
    as {
        Number u32 "[0-9]+"
        String String "[a-zA-Z]+"
    }
}

impl FromStr for Input {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // ToDo: change this to fit your input
        let (numbers, strings) = Token::parse(input)?;
        Ok(Self {
            numbers: get_numbers(numbers),
            strings: get_strings(strings),
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

fn get_strings(arr: Vec<Token>) -> Vec<String> {
    arr.into_iter()
        .filter_map(|time| match time {
            Token::String(val) => Some(val),
            _ => None,
        })
        .collect_vec()
}
