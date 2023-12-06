use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

use colored::Colorize;
use itertools::Itertools;

use crate::types::{Input, MapKey};

pub async fn solve() -> Result<(), String> {
    let location = find_closest_location()?;
    println!(
        "{prefix}: {value}",
        prefix = "Seeds file".blue(),
        value = DATA_FILE.to_string().yellow()
    );
    println!(
        "{prefix}: {question}",
        prefix = "  Question".blue(),
        question = "What is the lowest location number that corresponds to any of the initial seed numbers?".yellow().bold()
    );
    println!(
        "{prefix}: {answer}",
        prefix = "    Answer".blue(),
        answer = location.to_string().green().bold()
    );
    Ok(())
}

pub const DATA_FILE: &str = "./data/input.txt";

pub fn parse_input<T>(file: &str) -> Result<T, String>
where
    T: FromStr,
    T::Err: Debug,
{
    // Open file reader
    let file = File::open(file).map_err(|err| format!("Failed to open cards file!\n{err}"))?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .expect("Input file read failed!");
    Ok(data.parse().expect("Failed to parse input!"))
}

pub fn find_closest_location() -> Result<u64, String> {
    let input: Input = parse_input(DATA_FILE)?;
    let (
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humid,
        humid_to_loc,
    ) = (
        |id| {
            input
                .maps
                .get(&MapKey::new("seed", "soil"))
                .unwrap()
                .map(id)
        },
        |id| {
            input
                .maps
                .get(&MapKey::new("soil", "fertilizer"))
                .unwrap()
                .map(id)
        },
        |id| {
            input
                .maps
                .get(&MapKey::new("fertilizer", "water"))
                .unwrap()
                .map(id)
        },
        |id| {
            input
                .maps
                .get(&MapKey::new("water", "light"))
                .unwrap()
                .map(id)
        },
        |id| {
            input
                .maps
                .get(&MapKey::new("light", "temperature"))
                .unwrap()
                .map(id)
        },
        |id| {
            input
                .maps
                .get(&MapKey::new("temperature", "humidity"))
                .unwrap()
                .map(id)
        },
        |id| {
            input
                .maps
                .get(&MapKey::new("humidity", "location"))
                .unwrap()
                .map(id)
        },
    );
    let mapped = input
        .seeds
        .clone()
        .into_iter()
        .zip(
            input
                .seeds
                .into_iter()
                .map(seed_to_soil)
                .map(soil_to_fert)
                .map(fert_to_water)
                .map(water_to_light)
                .map(light_to_temp)
                .map(temp_to_humid)
                .map(humid_to_loc),
        )
        .collect_vec();

    Ok(mapped.into_iter().map(|(_, loc)| loc).min().unwrap())
}
