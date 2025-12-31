pub mod preference_loading {
    pub fn every_nth(population: u32, fraction: usize) -> Vec<[u8; 3]> {
        let pop = population as usize;
        // Loading preference data into use
        // Assumes transitive and complete preferences (total ordering)
        // TODO: Perhaps it's better to have key value pair from a preference to the number of holders
        // of that preference
        let mut preferences: Vec<[u8; 3]> = Vec::with_capacity(pop);
        for n in 0..pop {
            preferences.push(if n % fraction == 0 {
                [0, 1, 2]
            } else {
                [1, 0, 2]
            });
        }

        preferences
    }
}
