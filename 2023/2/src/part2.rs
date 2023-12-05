use std::cmp::max;

use colored::Colorize;

use crate::{
    part1::{load_games, DATA_FILE},
    types::CubeSet,
};

pub async fn solve() -> Result<(), String> {
    let sum: i32 = get_minimal_sets_for_games()?
        .iter()
        .map(|set| set.red * set.blue * set.green)
        .sum();

    println!(
        "{prefix}: {value}",
        prefix = "Games file".blue(),
        value = DATA_FILE.to_string().yellow()
    );

    println!(
        "{prefix}: {question}",
        prefix = "  Question".blue(),
        question = "What is the sum of the power of fewest number of cubes for each game?"
            .yellow()
            .bold()
    );
    println!(
        "{prefix}: {answer}",
        prefix = "    Answer".blue(),
        answer = sum.to_string().green().bold()
    );
    Ok(())
}

fn get_minimal_sets_for_games() -> Result<Vec<CubeSet>, String> {
    Ok(load_games()?
        .into_iter()
        .map(|game| {
            game.rounds
                .iter()
                .fold(CubeSet::default(), |mut set, round| {
                    set.red = max(round.red, set.red);
                    set.green = max(round.green, set.green);
                    set.blue = max(round.blue, set.blue);
                    set
                })
        })
        .collect())
}
