use colored::Colorize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub async fn solve() {
    println!(
        "Calibration document in: {file}\n",
        file = DATA_FILE.yellow()
    );

    let sum = sum_calibration_data()
        .map_err(|err| panic!("{err}"))
        .unwrap();

    println!(
        "{prefix}: {question}",
        prefix = "Question".blue(),
        question = "What is the sum of all of the calibration values?"
            .yellow()
            .bold()
    );
    println!(
        "{prefix}: {answer}",
        prefix = "  Answer".blue(),
        answer = sum.to_string().green().bold()
    );
}

const DATA_FILE: &str = "./data/input.txt";

fn sum_calibration_data() -> Result<u32, String> {
    // Open file reader
    let file = File::open(DATA_FILE)
        .map_err(|err| format!("Failed to open calibration document!\n{err}"))?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut sum = 0;
    loop {
        // Read line of calibration data
        let read = reader
            .read_line(&mut line)
            .map_err(|err| format!("Failed to read calibration document!\n{err}"))?;

        // Now parse them
        let mut numbers = line.chars().filter_map(|c| char::to_digit(c, 10));
        let first = numbers.next().unwrap_or(0);
        let last = numbers.next_back().unwrap_or(first);

        // Update sum
        sum += first * 10 + last;

        if read == 0 {
            // Reached EOF, stop reading
            break;
        }
        line.clear();
    }

    Ok(sum)
}
