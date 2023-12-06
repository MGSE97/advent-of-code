use colored::Colorize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::types::Card;

pub async fn solve() -> Result<(), String> {
    let sum = sum_card_points()?;

    println!(
        "{prefix}: {value}",
        prefix = "Cards file".blue(),
        value = DATA_FILE.to_string().yellow()
    );
    println!(
        "{prefix}: {question}",
        prefix = "  Question".blue(),
        question = "How many points are cards worth in total?".yellow().bold()
    );
    println!(
        "{prefix}: {answer}",
        prefix = "    Answer".blue(),
        answer = sum.to_string().green().bold()
    );
    Ok(())
}

pub const DATA_FILE: &str = "./data/input.txt";

pub fn load_cards() -> Result<Vec<Card>, String> {
    // Open file reader
    let file = File::open(DATA_FILE).map_err(|err| format!("Failed to open cards file!\n{err}"))?;
    let mut reader = BufReader::new(file);

    let mut cards = Vec::<Card>::new();
    let mut line = String::new();
    loop {
        let read = reader
            .read_line(&mut line)
            .map_err(|err| format!("Failed to read cards file!\n{err}"))?;

        if let Ok(card) = line.parse() {
            cards.push(card);
        }

        if read == 0 {
            break;
        }
        line.clear();
    }
    Ok(cards)
}

pub fn sum_card_points() -> Result<u32, String> {
    let cards = load_cards()?;
    Ok(cards.iter().map(compute_card_points).sum())
}

pub fn compute_card_points(card: &Card) -> u32 {
    let mut points = None;
    for number in &card.card_numbers {
        if card.winning_numbers.contains(number) {
            points = Some(match points {
                Some(val) => val * 2,
                None => 1,
            });
        }
    }
    points.unwrap_or_default()
}
