// Shared functions for solutions

use std::fmt::Display;

use itertools::Itertools;
use lib::Matrix2D;

use crate::types::{Input, Tile, TileData};

// Move them here once you"re done with both parts
pub fn show_matrix<T>(matrix: &Matrix2D<T>)
where
    T: Display,
{
    println!(
        "\r{}",
        matrix
            .iter()
            .map(|r| r.iter().map(|t| t.to_string()).join(""))
            .join("\n")
    );
}

/**
 * Fills distance matrix using flood fill algorithm
*/
pub fn fill_distances(
    tiles: &mut Matrix2D<TileData>,
    col: usize,
    row: usize,
) -> Result<(usize, usize), String> {
    // Create buffer to insert next checks
    let mut fill = Vec::<(usize, usize, Option<(usize, usize)>)>::with_capacity(128);
    // Set starting point
    fill.push((col, row, None));
    // Set distance to 0
    let mut distance = 0;
    let mut longest = None;

    loop {
        let mut drain = fill.clone();
        fill.clear();

        for (fx, fy, previous) in drain.drain(0..) {
            if let Ok(mut data) = tiles.get_mut(fx, fy) {
                if data.distance.is_none() && !matches!(data.tile, Tile::Ground) {
                    let (x, y) = (fx as i32, fy as i32);
                    if let Some((px, py)) = previous {
                        let (cpx, cpy) = (px as i32, py as i32);
                        if !match data.tile {
                            Tile::PipeNtS => (x, y - 1) == (cpx, cpy) || (x, y + 1) == (cpx, cpy),
                            Tile::PipeEtW => (x + 1, y) == (cpx, cpy) || (x - 1, y) == (cpx, cpy),
                            Tile::PipeNtE => (x, y - 1) == (cpx, cpy) || (x + 1, y) == (cpx, cpy),
                            Tile::PipeNtW => (x, y - 1) == (cpx, cpy) || (x - 1, y) == (cpx, cpy),
                            Tile::PipeStW => (x, y + 1) == (cpx, cpy) || (x - 1, y) == (cpx, cpy),
                            Tile::PipeStE => (x, y + 1) == (cpx, cpy) || (x + 1, y) == (cpx, cpy),
                            _ => false,
                        } {
                            // Cannot connect
                            continue;
                        }
                    }

                    data.distance = Some(distance);
                    data.previous = previous;
                    longest = Some((fx, fy));

                    match data.tile {
                        Tile::PipeNtS => {
                            safe_push(&mut fill, x, y - 1, (fx, fy));
                            safe_push(&mut fill, x, y + 1, (fx, fy));
                        }
                        Tile::PipeEtW => {
                            safe_push(&mut fill, x + 1, y, (fx, fy));
                            safe_push(&mut fill, x - 1, y, (fx, fy));
                        }
                        Tile::PipeNtE => {
                            safe_push(&mut fill, x, y - 1, (fx, fy));
                            safe_push(&mut fill, x + 1, y, (fx, fy));
                        }
                        Tile::PipeNtW => {
                            safe_push(&mut fill, x, y - 1, (fx, fy));
                            safe_push(&mut fill, x - 1, y, (fx, fy));
                        }
                        Tile::PipeStW => {
                            safe_push(&mut fill, x, y + 1, (fx, fy));
                            safe_push(&mut fill, x - 1, y, (fx, fy));
                        }
                        Tile::PipeStE => {
                            safe_push(&mut fill, x, y + 1, (fx, fy));
                            safe_push(&mut fill, x + 1, y, (fx, fy));
                        }
                        Tile::Start => {
                            safe_push(&mut fill, x, y + 1, (fx, fy));
                            safe_push(&mut fill, x, y - 1, (fx, fy));
                            safe_push(&mut fill, x + 1, y, (fx, fy));
                            safe_push(&mut fill, x - 1, y, (fx, fy));
                        }
                        _ => {}
                    }
                }
            }
        }

        if fill.is_empty() {
            break;
        }

        distance += 1;
    }

    let end_pos = longest.unwrap_or_default();

    // Mark longes path
    let mut end = true;
    let mut previous = None;
    while let Some((x, y)) = longest {
        if let Ok(mut data) = tiles.get_mut(x, y) {
            data.path = true;
            data.end = end;
            longest = data.previous;
            previous = Some((x, y));
            end = false;
        } else {
            break;
        }
    }
    if let Some((x, y)) = previous {
        if let Ok(mut data) = tiles.get_mut(x, y) {
            data.path = true;
        }
    }

    Ok(end_pos)
}

fn safe_push<T>(
    fill: &mut Vec<(usize, usize, Option<(usize, usize)>)>,
    col: T,
    row: T,
    previous: (usize, usize),
) where
    T: TryInto<usize>,
{
    if let Ok(col) = col.try_into() {
        if let Ok(row) = row.try_into() {
            fill.push((col, row, Some(previous)));
        }
    }
}

pub fn get_starting_position(input: &Input) -> (usize, usize) {
    let mut x = 0;
    let y = input
        .tiles
        .iter()
        .position(
            |row| match row.iter().position(|data| data.tile == Tile::Start) {
                Some(pos) => {
                    x = pos;
                    true
                }
                None => false,
            },
        )
        .unwrap_or_else(|| panic!("Start not found!"));
    (x, y)
}
