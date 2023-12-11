use itertools::Itertools;
use lib::*;

use crate::types::Input;

solve! {
    files << "Input" "./data/input.txt"
    "What is the sum of these extrapolated values?"
    "The sum is {answer}.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(input: Input) -> impl Into<Answer> {
    input
        .values
        .iter()
        .map(|val| {
            let mut lasts = Vec::new();
            let mut deltas = val.clone();

            // Check for all zeros in deltas
            while deltas.iter().sum::<i64>() != 0 {
                // Get last delta to add to next prediction
                lasts.push((deltas.clone(), deltas.last().unwrap().to_owned()));
                // Compute new deltas
                // We go reverse from before last and compute differences
                let mut iter = deltas.iter();
                let mut prev = iter.next().unwrap().to_owned();
                deltas = iter
                    .map(|&val| {
                        // Compute delta
                        let delta = prev - val;
                        let next = delta;
                        /*let next = match delta < 0 {
                            true => -delta,
                            false => delta,
                        };*/
                        prev = next;
                        next
                    })
                    .collect_vec();
            }
            lasts
                .clone()
                .into_iter()
                .rev()
                .reduce(|prev, val| {
                    println!("{:?} {}", val.0, val.1 + prev.1);
                    (val.0, val.1 + prev.1)
                })
                .unwrap_or_default();
            // Compute prediction
            lasts
                .into_iter()
                .rev()
                .map(|val| val.1)
                .reduce(|prev, val| val + prev)
                .unwrap_or_default()
        })
        .sum::<i64>()
}
