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

    run_world(population, timesteps);
}

#[time_function]
fn run_world(population: usize, timesteps: u32) {
    let mut world: World<u32> = World {
        population,
        kpis: vec![0],
        delegation: HashMap::new(),
        proportion_of_adversarials: 0.1,
        timesteps,
    };

    World::<u32>::simulation(&mut world);
}

#[derive(Debug)]
struct World<T> {
    timesteps: u32,
    population: usize,
    kpis: Vec<usize>,
    delegation: HashMap<T, T>,
    proportion_of_adversarials: f32,
}

impl<T> World<T> {
    fn simulation(&mut self) {
        for i in 0..self.timesteps {
            println!("Timestep: {i}");
            let _prediction = self.simulate_prediction_market();
            println!("Prediction: {_prediction}");
            let voting_results = self.simulate_voting();
            self.simulate_world(&voting_results);
        }
    }

    fn simulate_world(&mut self, voting_results: &HashMap<String, usize>) {
        // Winner
        let (name, votes) = voting_results.iter().max_by_key(|entry| entry.1).unwrap();

        println!("Winner: {} with {} votes", name, votes);

        self.kpis[0] += match name.as_str() {
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
            "Mixed" => 10,
            "Raddish" => 20,
            "Potato" => 30,
            _ => 10,
        }
    }

    fn simulate_voting(&self) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        map.insert("Mixed".to_string(), self.population / 4);
        map.insert("Raddish".to_string(), self.population / 4);
        map.insert("Potato".to_string(), self.population / 2);
        map
    }
}
