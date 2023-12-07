use lib::*;

solve! {
    files << "Input" "./data/input.txt"
    "When is the mathematic of tears?"
    "In {answer} episode.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: String) -> impl Into<Answer> {
    // Code from here
    12
}
