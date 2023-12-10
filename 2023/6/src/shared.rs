// Shared functions for solutions

pub fn get_distance(wait: u64, time: u64) -> u64 {
    let speed = wait;
    let move_time = time - wait;
    move_time * speed
}

pub fn count_ways_to_win(distance: u64, time: u64) -> usize {
    (0..=time)
        .filter_map(|wait| {
            let our_distance = get_distance(wait, time);
            match our_distance > distance {
                true => Some((wait, our_distance)),
                false => None,
            }
        })
        .count()
}
