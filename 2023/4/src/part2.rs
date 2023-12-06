use std::{cell::RefCell, collections::BTreeMap};

use colored::Colorize;

use crate::{
    part1::{load_cards, DATA_FILE},
    types::{Card, CardInfo},
};

pub async fn solve() -> Result<(), String> {
    let sum = sum_winning_cards()?;

    println!(
        "{prefix}: {value}",
        prefix = "Cards file".blue(),
        value = DATA_FILE.to_string().yellow()
    );
    println!(
        "{prefix}: {question}",
        prefix = "  Question".blue(),
        question = "How many total scratchcards do you end up with?"
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

fn sum_winning_cards() -> Result<u32, String> {
    // Create map with count of winning numbers
    // and counter starting with 1 original card
    let cards = BTreeMap::<u32, CardInfo>::from_iter(load_cards()?.iter().map(|card| {
        (
            card.id,
            CardInfo {
                points: count_card_wins(card),
                count: RefCell::new(1),
            },
        )
    }));

    // Go through each card
    for (id, info) in cards.iter() {
        if info.points > 0 {
            // If it has any points
            let copies: u32 = *info.count.borrow();
            for i in 0..info.points {
                // Find next card id we want to copy
                let copy_id: u32 = *id + i as u32 + 1;
                if let Some(copy) = cards.get(&copy_id) {
                    // Increase it's counter by count of this card
                    copy.count.replace_with(|&mut val| val + copies);
                }
            }
        }
    }

    Ok(cards.values().map(|info| info.count.take()).sum())
}

pub fn count_card_wins(card: &Card) -> usize {
    card.card_numbers
        .iter()
        .filter(|number| card.winning_numbers.contains(number))
        .count()
}
