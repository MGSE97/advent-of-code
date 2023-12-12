use lib::*;

use crate::types::Input;

solve! {
    files << "Input" "./data/input.sm.txt"
    "When is the mathematic of tears?"
    "In {answer} episode.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(_input: Input) -> impl Into<Answer> {
    // Code from here
    12
}
