use lib::*;

use crate::types::Input;

solve! {
    files << "Input" "./data/input.sm.txt"
    "What do you get if you multiply these numbers together?"
    "We get {answer} ways to beat record.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    input
        .distances
        .into_iter()
        .zip(input.times)
        .map(|(time, distance)| {
            (0..time)
                .filter_map(|wait| {
                    let our_distance = get_distance(wait, time);
                    match our_distance > distance {
                        true => Some((wait, our_distance)),
                        false => None,
                    }
                })
                .count()
        })
        .reduce(|a, b| a * b)
        .unwrap_or_default()
}

pub fn get_distance(wait: u32, time: u32) -> u32 {
    let speed = wait;
    let move_time = time - wait;
    move_time * speed
}
