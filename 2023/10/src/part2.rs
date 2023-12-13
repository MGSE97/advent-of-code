use lib::*;

use crate::types::Input;

solve! {
    files << "Input" "./data/input2.sm2.txt"
    "How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?"
    "It takes {answer} steps.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(_input: Input) -> impl Into<Answer> {
    // Code from here
    252
}
