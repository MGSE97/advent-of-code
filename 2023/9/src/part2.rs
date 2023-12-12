use itertools::Itertools;
use lib::*;

use crate::{shared::compute_edge_sum, types::Input};

solve! {
    files << "Input" "./data/input.txt"
    "What is the sum of these extrapolated values?"
    "The sum is {answer}.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    input
        .values
        .iter()
        .map(|history| {
            // We just reverse input. It behaves same as before.
            let reversed = history.iter().cloned().rev().collect_vec();
            compute_edge_sum(reversed)
        })
        .sum::<i64>()
}
