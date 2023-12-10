use lib::parse_args_solution;

mod shared;

mod part1;
mod types1;

mod part2;
mod types2;

/// Run with part as argument to solve task.
/// ```sh
/// cargo run -- solve part 1
/// cargo run -- solve part 2
/// ```
#[tokio::main]
async fn main() -> Result<(), String> {
    let result = match parse_args_solution()? {
        1 => part1::solve().await?,
        2 => part2::solve().await?,
        _ => panic!("Solution doesn't exists!"),
    };
    println!("{result}");
    Ok(())
}
