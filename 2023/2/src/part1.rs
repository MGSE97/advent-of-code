use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use colored::Colorize;

use crate::types::{CubeSet, Game};

pub async fn solve() -> Result<(), String> {
    let sum: i32 = get_valid_games()?.iter().map(|game| game.id).sum();
    println!(
        "{prefix}: {value}",
        prefix = "Games file".blue(),
        value = DATA_FILE.to_string().yellow()
    );
    println!(
        "{prefix}: Bag{value}",
        prefix = " Available".blue(),
        value = AVAILABLE_CUBES
    );

    println!(
        "{prefix}: {question}",
        prefix = "  Question".blue(),
        question = "What is the sum of the IDs of valid games?".yellow().bold()
    );
    println!(
        "{prefix}: {answer}",
        prefix = "    Answer".blue(),
        answer = sum.to_string().green().bold()
    );
    Ok(())
}

pub const DATA_FILE: &str = "./data/input.txt";
static AVAILABLE_CUBES: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

pub fn load_games() -> Result<Vec<Game>, String> {
    // Open file reader
    let file = File::open(DATA_FILE).map_err(|err| format!("Failed to open games file!\n{err}"))?;
    let mut reader = BufReader::new(file);

    let mut games = Vec::new();
    let mut line = String::new();
    loop {
        let read = reader
            .read_line(&mut line)
            .map_err(|err| format!("Failed to read games file!\n{err}"))?;

        if let Ok(game) = line.parse() {
            games.push(game);
        }

        if read == 0 {
            break;
        }
        line.clear();
    }

    Ok(games)
}

fn get_valid_games() -> Result<Vec<Game>, String> {
    Ok(load_games()?
        .into_iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.red <= AVAILABLE_CUBES.red
                    && round.green <= AVAILABLE_CUBES.green
                    && round.blue <= AVAILABLE_CUBES.blue
            })
        })
        .collect())
}
