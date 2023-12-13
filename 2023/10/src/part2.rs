use lib::*;

use crate::{
    shared::{fill_distances, get_starting_position, show_matrix},
    types::{Input, TileData},
};

solve! {
    files << "Input" "./data/input2.sm2.txt"
    "How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?"
    "It takes {answer} steps.",
    answer = get_answer(files.parse_file("Input")?).into()
}

pub fn get_answer(mut input: Input) -> impl Into<Answer> {
    // Get strating point position
    let (sx, sy) = get_starting_position(&input);
    println!("\n{:5} (x = {sx: >3}, y = {sy: >3})", "Start".yellow());

    // Create distances map from starting position
    if let Ok((ex, ey)) = fill_distances(&mut input.tiles, sx, sy) {
        println!("{:5} (x = {ex: >3}, y = {ey: >3})", "End".red());
        follow_pipes(&mut input.tiles, (ex, ey))
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

fn follow_pipes(tiles: &mut Matrix2D<TileData>, end: (usize, usize)) {
    let end_data = tiles.get(end.0, end.1).unwrap();
    let (mut x, mut y) = tiles
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, col)| {
                match col.distance == Some(end_data.distance.unwrap() - 1)
                    && (x, y) != end_data.previous.unwrap_or_default()
                {
                    true => Some((x, y)),
                    false => None,
                }
            })
        })
        .unwrap_or_default();

    let mut previous = end;
    while let Ok(mut data) = tiles.get_mut(x, y) {
        let next = data.previous;
        data.previous = Some(previous);
        previous = (x, y);
        if let Some(previous) = next {
            (x, y) = previous
        } else {
            break;
        }
    }

    let (mut x, mut y) = end;
    loop {
        if let Ok(mut data) = tiles.get_mut(x, y) {
            data.end = true;
        }
        show_matrix(tiles);
        if let Ok(mut data) = tiles.get_mut(x, y) {
            data.end = false;
            if let Some(previous) = data.previous {
                (x, y) = previous
            } else {
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        print!("\n{esc}c", esc = 27 as char);
    }
}
