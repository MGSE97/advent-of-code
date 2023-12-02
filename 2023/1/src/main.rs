use colored::Colorize;
use phf::{phf_map, Map};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const DATA_FILE: &str = "./data/input.txt";
static SPELLED_NUMBERS: Map<&str, u32> = phf_map! {
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

pub fn main() {
    println!(
        "Calibration document in: {file}\n",
        file = DATA_FILE.yellow()
    );
    let sum = sum_calibration_data()
        .map_err(|err| panic!("{err}"))
        .unwrap();
    println! {
        "{question}\n{answer}: {sum}\n",
        question="What is the sum of all of the calibration values?".yellow().bold(),
        answer="Sum of all calibration values is".blue(),
        sum=sum.to_string().green().bold()
    };
}

fn sum_calibration_data() -> Result<u32, String> {
    // Open file reader
    let file = File::open(DATA_FILE)
        .map_err(|err| format!("Failed to open calibration document!\n{err}"))?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut first_number = String::new();
    let mut last_number = String::new();
    let keys: HashSet<&str> = SPELLED_NUMBERS.keys().map(|key| key.to_owned()).collect();
    let mut sum = 0;
    loop {
        // Read line of calibration data
        let read = reader
            .read_line(&mut line)
            .map_err(|err| format!("Failed to read calibration document!\n{err}"))?;

        // Get first and last numbers
        let first = line
            .chars()
            .filter_map(|value| {
                find_digit(value).or_else(|| find_spelled_number(&keys, &mut first_number, value))
            })
            .next()
            .unwrap_or(0);

        let last = line
            .chars()
            .rev()
            .filter_map(|value| {
                find_digit(value)
                    .or_else(|| find_spelled_number_rev(&keys, &mut last_number, value))
            })
            .next()
            .unwrap_or(first);

        // Update sum
        sum += first * 10 + last;

        if read == 0 {
            // Reached EOF, stop reading
            break;
        }

        line.clear();
        first_number.clear();
        last_number.clear();
    }

    Ok(sum)
}

fn find_digit(value: char) -> Option<u32> {
    char::to_digit(value, 10)
}

fn find_spelled_number(keys: &HashSet<&str>, buff: &mut String, value: char) -> Option<u32> {
    // If char not digit, add previous valid charactes as prefix
    let mut search = format!("{buff}{value}");
    // While we have some prefix to search
    while !search.is_empty() {
        // Search for key, that starts with it
        match keys.iter().any(|key| key.starts_with(&search)) {
            // If found, check if it's whole key
            true => match SPELLED_NUMBERS.get(&search) {
                Some(key) => {
                    // Is whole key
                    // Return mapped value of current number
                    return Some(*key);
                }
                None => {
                    // Is not whole key
                    // Add search as number prefix
                    *buff = search;
                    return None;
                }
            },
            false => {
                // Not in any key
                // Remove first character from search and try again
                search.remove(0);
            }
        }
    }
    // Clear buffer on empty search
    // No key starts with this char
    buff.clear();
    None
}

fn find_spelled_number_rev(keys: &HashSet<&str>, buff: &mut String, value: char) -> Option<u32> {
    // If char not digit, add previous valid charactes as prefix
    let mut search = format!("{value}{buff}");
    // While we have some postfix to search
    while !search.is_empty() {
        // Search for key, that starts with it
        match keys.iter().any(|key| key.ends_with(&search)) {
            // If found, check if it's whole key
            true => match SPELLED_NUMBERS.get(&search) {
                Some(key) => {
                    // Is whole key
                    // Return mapped value of current number
                    return Some(*key);
                }
                None => {
                    // Is not whole key
                    // Add search as number prefix
                    *buff = search;
                    return None;
                }
            },
            false => {
                // Not in any key
                // Remove last character from search and try again
                search.pop();
            }
        }
    }
    // Clear buffer on empty search
    // No key ends with this char
    buff.clear();
    None
}
