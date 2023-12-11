use itertools::Itertools;
use lib::*;

use crate::types::Input;

solve! {
    files << "Input" "./data/input.sm.txt"
    "When is the mathematic of tears?"
    "In {answer} episode.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    input
        .hands
        .into_iter()
        .map(|hand| {
            (
                hand.rank(),
                hand.cards.values().sorted().rev().cloned().collect_vec(),
                hand.bid,
            )
        })
        .sorted()
        /*.sorted_by(|(a_rank, ..), (b_rank, ..)| match a_rank <= b_rank {
            false => std::cmp::Ordering::Less,
            true => std::cmp::Ordering::Greater,
        })*/
        .enumerate()
        .map(|x| {
            println!("{x:?}");
            x
        })
        .map(|(i, (_, _, bid))| (i + 1) * bid)
        .reduce(|a, b| a + b)
        .unwrap_or_default()
}
