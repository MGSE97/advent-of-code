use lib::*;

use crate::{shared::count_ways_to_win, types1::Input};

solve! {
    files << "Input" "./data/input.txt"
    "What do you get if you multiply these numbers together?"
    "We get {answer} ways to beat record.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    input
        .distances
        .into_iter()
        .zip(input.times)
        .map(|(distance, time)| count_ways_to_win(distance, time))
        .reduce(|a, b| a * b)
        .unwrap_or_default()
}
