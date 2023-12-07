use colored::Colorize;
use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    part1::{parse_input, DATA_FILE},
    types::{Input, MapKey},
};

pub async fn solve() -> Result<(), String> {
    let location = find_closest_location_in_range()?;
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

fn find_closest_location_in_range() -> Result<u64, String> {
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
        input.maps.get(&MapKey::new("seed", "soil")).unwrap(),
        input.maps.get(&MapKey::new("soil", "fertilizer")).unwrap(),
        input.maps.get(&MapKey::new("fertilizer", "water")).unwrap(),
        input.maps.get(&MapKey::new("water", "light")).unwrap(),
        input
            .maps
            .get(&MapKey::new("light", "temperature"))
            .unwrap(),
        input
            .maps
            .get(&MapKey::new("temperature", "humidity"))
            .unwrap(),
        input
            .maps
            .get(&MapKey::new("humidity", "location"))
            .unwrap(),
    );

    let max_loc = humid_to_loc
        .ranges
        .par_iter()
        .max_by_key(|range| range.target_upper)
        .map(|range| range.target_upper)
        .unwrap();

    let seed_ranges = input
        .seeds
        .chunks_exact(2)
        .filter_map(|chunk| match chunk.len() == 2 {
            true => {
                let seed = chunk[0];
                let count = chunk[1];
                Some((seed, seed + count))
            }
            false => None,
        })
        .collect_vec();

    let min_loc = (0..=max_loc)
        .into_par_iter()
        .map(|loc| (loc, humid_to_loc.rmap(loc)))
        .map(|(loc, id)| (loc, temp_to_humid.rmap(id)))
        .map(|(loc, id)| (loc, light_to_temp.rmap(id)))
        .map(|(loc, id)| (loc, water_to_light.rmap(id)))
        .map(|(loc, id)| (loc, fert_to_water.rmap(id)))
        .map(|(loc, id)| (loc, soil_to_fert.rmap(id)))
        .map(|(loc, id)| (loc, seed_to_soil.rmap(id)))
        .find_first(|&(_, id)| seed_ranges.iter().any(|&(min, max)| min <= id && id <= max))
        .map(|(loc, _)| loc)
        .unwrap();

    /*let min_loc = input
    .seeds
    .chunks_exact(2)
    .take(1)
    .filter_map(|chunk| match chunk.len() == 2 {
        true => {
            let seed = chunk[0];
            let count = chunk[1];
            (seed..=(seed + count))
                .into_par_iter()
                .map(|id| seed_to_soil.map(id))
                .map(|id| soil_to_fert.map(id))
                .map(|id| fert_to_water.map(id))
                .map(|id| water_to_light.map(id))
                .map(|id| light_to_temp.map(id))
                .map(|id| temp_to_humid.map(id))
                .map(|id| humid_to_loc.map(id))
                .min()
        }
        false => None,
    })
    .min()
    .unwrap();*/

    Ok(min_loc)
}
