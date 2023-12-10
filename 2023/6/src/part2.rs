use lib::*;

use crate::{shared::count_ways_to_win, types2::Input};

solve! {
    files << "Input" "./data/input.txt"
    "How many ways can you beat the record in this one much longer race?"
    "We get {answer} ways to beat record.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    count_ways_to_win(input.distance, input.time)
}
