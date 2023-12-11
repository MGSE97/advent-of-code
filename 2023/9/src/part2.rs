use lib::*;

use crate::types::Input;

solve! {
    files << "Input" "./data/input.sm.txt"
    "For how many seconds he has been leaf on the wind?"
    "He was leaf for {answer} seconds.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    // Code from here
    252
}
