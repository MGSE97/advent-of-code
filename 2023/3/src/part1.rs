use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

use colored::Colorize;
use itertools::Itertools;

use crate::types::{Matrix, Part};

pub async fn solve() -> Result<(), String> {
    let sum = process_matrix()?;
    println!(
        "{prefix}: {value}",
        prefix = "Engine file".blue(),
        value = DATA_FILE.to_string().yellow()
    );
    println!(
        "{prefix}: {question}",
        prefix = "   Question".blue(),
        question = "What is the sum of all of the part numbers in the engine schematic?"
            .yellow()
            .bold()
    );
    println!(
        "{prefix}: {answer}",
        prefix = "     Answer".blue(),
        answer = sum.to_string().green().bold()
    );
    Ok(())
}

pub const DATA_FILE: &str = "./data/input.txt";

pub fn load_matrix() -> Result<Matrix<Part>, String> {
    // Open file reader
    let file =
        File::open(DATA_FILE).map_err(|err| format!("Failed to open schematic file!\n{err}"))?;
    let mut reader = BufReader::new(file);

    let mut matrix = Matrix::default();
    let mut line = String::new();
    loop {
        let read = reader
            .read_line(&mut line)
            .map_err(|err| format!("Failed to read schematic file!\n{err}"))?;

        let mut number = None;
        let row = line
            .trim_end()
            .chars()
            .map(|char| match (char, char.to_digit(10), &number) {
                (_, Some(val), None) => {
                    let val_ref = Rc::new(RefCell::new(val));
                    number = Some(val_ref.clone());
                    Part::Number(val_ref)
                }
                (_, Some(val), Some(num)) => {
                    num.replace_with(|&mut n| n * 10 + val);
                    Part::Number(num.clone())
                }
                ('.', _, _) => {
                    number = None;
                    Part::None
                }
                (_, _, _) => {
                    number = None;
                    Part::Symbol
                }
            })
            .collect();

        matrix.add(row);
        if read == 0 {
            break;
        }
        line.clear();
    }
    Ok(matrix)
}

fn process_matrix() -> Result<u32, String> {
    let matrix = load_matrix()?;
    let (r, c) = matrix.size();
    let (rows, cols) = (r as i32, c as i32);
    let mut sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            if let Part::Symbol = safe_get(&matrix, row, col) {
                let items: u32 = vec![
                    safe_get_num(&matrix, row + 1, col + 1),
                    safe_get_num(&matrix, row + 1, col),
                    safe_get_num(&matrix, row + 1, col - 1),
                    safe_get_num(&matrix, row, col + 1),
                    safe_get_num(&matrix, row, col - 1),
                    safe_get_num(&matrix, row - 1, col + 1),
                    safe_get_num(&matrix, row - 1, col),
                    safe_get_num(&matrix, row - 1, col - 1),
                ]
                .iter()
                .unique()
                .sum();

                sum += items;
            }
        }
    }
    Ok(sum)
}

pub fn safe_get_num(matrix: &Matrix<Part>, row: i32, col: i32) -> u32 {
    if let Part::Number(num) = safe_get(matrix, row, col) {
        num.replace_with(|&mut x| x)
    } else {
        0
    }
}

pub fn safe_get(matrix: &Matrix<Part>, row: i32, col: i32) -> &Part {
    match matrix.get(row, col) {
        Some(part) => part,
        None => &Part::None,
    }
}
