use std::{fs::File, io::{BufReader, BufRead}};
use colored::Colorize;

const DATA_FILE: &str = "./data/input.txt";

pub fn main() {
    println!("Calibration document in: {file}\n", file=DATA_FILE.yellow());
    let sum = sum_calibration_data().map_err(|err| panic!("{err}")).unwrap();
    println!{
        "{question}\n{answer}: {sum}\n", 
        question="What is the sum of all of the calibration values?".yellow().bold(), 
        answer="Sum of all calibration values is".blue(), 
        sum=sum.to_string().green().bold()
    };
}

fn sum_calibration_data() -> Result<u32, String> {
    // Open file reader
    let file = File::open(DATA_FILE).map_err(|err| format!("Failed to open calibration document!\n{err}"))?;
    let mut reader = BufReader::new(file);
    
    let mut line = String::new();
    let mut sum = 0;
    loop {
        // Read line of calibration data
        let read = reader.read_line(&mut line).map_err(|err| format!("Failed to read calibration document!\n{err}"))?;
        
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
    }

    Ok(sum)
}
