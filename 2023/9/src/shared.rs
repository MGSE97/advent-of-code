use itertools::Itertools;

// Shared functions for solutions
pub fn compute_edge_sum(history: Vec<i64>) -> i64 {
    let mut edge = Vec::new();
    let mut deltas = history;

    // Check for all zeros in deltas
    while deltas.iter().sum::<i64>() != 0 {
        // Get last delta value and add it to edge value
        edge.push(*deltas.last().unwrap());

        // Compute new deltas
        // We go from left to right, and compute difference
        deltas = deltas
            .iter()
            .tuple_windows()
            .map(|(left, right)| right - left)
            .collect();
    }

    // Compute prediction, by making sum of all edge values
    edge.into_iter()
        .reduce(|prev, val| val + prev)
        .unwrap_or_default()
}
