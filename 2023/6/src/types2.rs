// Types and impls for part 2
use std::str::FromStr;

use itertools::Itertools;
use lib::impl_parse;

#[derive(Debug, Clone)]
pub struct Input {
    pub time: u64,
    pub distance: u64,
}

impl_parse! {
    Token {
        Time:     SeparatedNumber
        Distance: SeparatedNumber
    }
    without r"[:\r\n]+"
    as {
        Time     "Time"
        Distance "Distance"
        SeparatedNumber String r"[0-9 ]+"
    }
}

impl FromStr for Input {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, time, _, distance) = Token::parse(input)?;
        Ok(Self {
            time: get_number(time),
            distance: get_number(distance),
        })
    }
}

fn get_number(token: Token) -> u64 {
    match token {
        Token::SeparatedNumber(val) => val.replace(' ', "").parse().unwrap_or_default(),
        _ => 0,
    }
}
