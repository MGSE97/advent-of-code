use lib::*;

use crate::types::Input;

solve! {
    files << "Input" "./data/input.txt"
    "How many steps are required to reach ZZZ from AAA?"
    "We need {answer} steps.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    // Code from here
    input.distance("AAA", "ZZZ")
}
