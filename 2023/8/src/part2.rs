use lib::*;

use crate::types::Input;

solve! {
    files << "Input" "./data/input.txt"
    "How many steps does it take before you're only on nodes that end with Z?"
    "We need {answer} steps.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    input.shadow_clones_distance('A', 'Z')
}
