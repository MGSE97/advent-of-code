use itertools::Itertools;
use lib::*;

use crate::types2::Input;

solve! {
    files << "Input" "./data/input.sm.txt"
    "What are the total winnings?"
    "Total winnings are {answer}.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    input
        .hands
        .into_iter()
        .map(|hand| (hand.rank(), hand.cards, hand.bid))
        .sorted()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid)
        .reduce(|a, b| a + b)
        .unwrap_or_default()
}
