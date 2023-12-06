use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

use colored::Colorize;
use itertools::Itertools;

use crate::{
    part1::{safe_get, safe_get_num, DATA_FILE},
    types::{Matrix, Part},
};

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
        question = "What is the sum of all of the gear ratios in your engine schematic?"
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
                ('*', _, _) => {
                    number = None;
                    Part::Symbol
                }
                (_, _, _) => {
                    number = None;
                    Part::None
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
                let items = vec![
                    safe_get_num(&matrix, row + 1, col + 1),
                    safe_get_num(&matrix, row + 1, col),
                    safe_get_num(&matrix, row + 1, col - 1),
                    safe_get_num(&matrix, row, col + 1),
                    safe_get_num(&matrix, row, col - 1),
                    safe_get_num(&matrix, row - 1, col + 1),
                    safe_get_num(&matrix, row - 1, col),
                    safe_get_num(&matrix, row - 1, col - 1),
                ]
                .into_iter()
                .filter(|x| *x > 0)
                .unique()
                .collect_vec();

                if items.len() == 2 {
                    let (a, b) = (items[0], items[1]);
                    sum += a * b;
                }
            }
        }
    }
    Ok(sum)
}
