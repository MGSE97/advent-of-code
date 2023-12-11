// Types and impls for both parts
use std::str::FromStr;

use itertools::Itertools;
use lib::impl_parse;

#[derive(Debug, Clone)]
pub struct Input {
    pub values: Vec<Vec<i64>>,
}

impl_parse! {
    Token {
        (Number+ EoL?)+
    }
    without r"[ ]+"
    as {
        Number i64 r"[\-0-9]+"
        EoL String r"[\r\n]+"
    }
}

impl FromStr for Input {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let values = Token::parse(input)?;
        Ok(Self {
            values: values
                .iter()
                .map(|(v, _)| v.iter().map(get_value).collect_vec())
                .collect_vec(),
        })
    }
}

fn get_value(token: &Token) -> i64 {
    match token {
        Token::Number(val) => Some(*val),
        _ => None,
    }
    .unwrap_or_default()
}
