use lib::*;

use crate::{
    shared::{fill_distances, get_starting_position, show_matrix},
    types::Input,
};

solve! {
    files << "Input" "./data/input.txt"
    "How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?"
    "It takes {answer} steps.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(mut input: Input) -> impl Into<Answer> {
    // Get strating point position
    let (x, y) = get_starting_position(&input);
    println!("\nStart: {x} x {y}\n");

    // Create distances map from starting position
    if let Err(err) = fill_distances(&mut input.tiles, x, y) {
        show_matrix(&input.tiles);
        return err;
    };

    show_matrix(&input.tiles);

    // Get maximal distance from distances
    (input
        .tiles
        .iter()
        .filter_map(|row| row.iter().filter_map(|data| data.distance.as_ref()).max())
        .max()
        .unwrap_or(&0)
        .to_owned())
    .to_string()
}
