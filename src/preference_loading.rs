pub fn every_nth(population: usize, fraction: usize) -> Vec<[u8; 3]> {
    // Loading preference data into use
    // Assumes transitive and complete preferences (total ordering)
    // TODO: Perhaps it's better to have key value pair from a preference to the number of holders
    // of that preference
    let mut preferences: Vec<[u8; 3]> = Vec::with_capacity(population);
    for n in 0..population {
        preferences.push(if n % fraction == 0 {
            [0, 1, 2]
        } else {
            [1, 0, 2]
        });
    }

    preferences
}
