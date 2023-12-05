use colored::Colorize;
use std::{fmt::Display, str::FromStr};

#[derive(Default)]
pub struct CubeSet {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl Display for CubeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({red}: {red_count}, {green}: {green_count}, {blue}: {blue_count})",
            red = "red".red(),
            green = "green".green(),
            blue = "blue".blue(),
            red_count = self.red.to_string().red().bold(),
            green_count = self.green.to_string().green().bold(),
            blue_count = self.blue.to_string().blue().bold(),
        )
    }
}

#[derive(Default)]
pub struct Game {
    pub id: i32,
    pub rounds: Vec<CubeSet>,
}

impl Game {
    pub fn add(&mut self, set: CubeSet) -> &Self {
        self.rounds.push(set);
        self
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !input.starts_with("Game") {
            return Err("Wrong data format!")?;
        }

        // Lets parse game data here
        let mut game = Self::default();

        let mut game_data = input.split(':');

        game.id = game_data
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        for round in game_data.next().unwrap().split(';') {
            let mut set = CubeSet::default();

            for cubes in round.split(',') {
                let mut data = cubes.split_whitespace();
                let count = data.next().unwrap().parse::<i32>().unwrap();
                let color = data.next().unwrap();
                match color {
                    "red" => set.red += count,
                    "green" => set.green += count,
                    "blue" => set.blue += count,
                    _ => return Err("Wrong color! ".to_string() + input)?,
                }
            }

            game.add(set);
        }

        Ok(game)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rounds = String::new();
        for round in &self.rounds {
            rounds += &format!("\tRound{round},\n");
        }
        write!(f, "Game {id} {{\n{rounds}}}", id = self.id)
    }
}
