use code_timing_macros::time_function;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;

mod preference;
mod preference_loading;

fn main() {
    let mut input = String::new();

    println!("Enter population:");
    io::stdin().read_line(&mut input).unwrap();
    let population: usize = input.trim().parse().unwrap();

    input.clear();
    println!("Enter timesteps:");
    io::stdin().read_line(&mut input).unwrap();
    let timesteps: u32 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter frequency of adversarials:");
    io::stdin().read_line(&mut input).unwrap();
    let proportion_of_adversarials: f32 = input.trim().parse().unwrap();
    run_world(population, timesteps, proportion_of_adversarials);
}

#[time_function]
fn run_world(population: usize, timesteps: u32, proportion_of_adversarials: f32) {
    let mut world: World = World {
        population,
        kpis: vec![0],
        proportion_of_adversarials,
        timesteps,
    };

    World::simulation(&mut world);
}

#[derive(Debug)]
struct World {
    timesteps: u32,
    population: usize,
    kpis: Vec<usize>,
    proportion_of_adversarials: f32,
}

impl World {
    fn simulation(&mut self) {
        for i in 0..self.timesteps {
            println!("Timestep: {i}");
            let _prediction = self.simulate_prediction_market();
            println!("Prediction: Best one yields {_prediction}");
            let voting_results = self.simulate_voting();
            self.simulate_world(&voting_results);
        }
    }

    fn simulate_world(&mut self, voting_results: &HashMap<String, u32>) {
        let (winner_name, winner_votes) =
            voting_results.iter().max_by_key(|entry| entry.1).unwrap();

        println!("Winner: {} with {} votes", winner_name, winner_votes);

        self.kpis[0] += match winner_name.as_str() {
            "Mixed" => 10,
            "Raddish" => 20,
            "Potato" => 30,
            _ => 10,
        };

        for kpi in &mut *self.kpis {
            println!("{kpi:?}");
        }
    }

    fn simulate_prediction_market(&self) -> i32 {
        match "Potato" {
            "Rotten Tomatos" => 4,
            "Mixed" => 10,
            "Raddish" => 20,
            "Potato" => 30,
            _ => 10,
        }
    }

    fn simulate_voting(&self) -> HashMap<String, u32> {
        let mut map = HashMap::new();
        map.insert(
            "Rotten Tomatos".to_string(),
            (self.population as f32 * self.proportion_of_adversarials).round() as u32,
        );
        map.insert(
            "Potato".to_string(),
            (self.population as f32 - self.population as f32 * self.proportion_of_adversarials)
                .round() as u32,
        );
        map
    }
}
