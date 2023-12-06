use colored::Colorize;
use std::env;

mod part1;
mod part2;
mod types;

/// Run with part as argument to solve task.
/// ```sh
/// cargo run -- solve part 1
/// cargo run -- solve part 2
/// ```
#[tokio::main]
async fn main() -> Result<(), String> {
    match env::args().filter_map(|arg| arg.parse::<i32>().ok()).next() {
        Some(1) => part1::solve().await?,
        Some(2) => part2::solve().await?,
        _ => {
            println!(
                "\n{err}\n{help}\n",
                err = "Specify part to solve!".red().bold(),
                help = "> cargo run -- solve part 1".yellow()
            );
            panic!("No part selected!")
        }
    };
    Ok(())
}
