use lib::*;

use crate::{shared::compute_edge_sum, types::Input};

solve! {
    files << "Input" "./data/input.txt"
    "What is the sum of these extrapolated values?"
    "The sum is {answer}.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    input.values.into_iter().map(compute_edge_sum).sum::<i64>()
}
