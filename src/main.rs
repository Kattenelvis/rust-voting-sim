use code_timing_macros::time_function;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::thread;

mod kpi;
mod preference_loading;

#[time_function]
fn main() {
    let mut world: World<u32> = World {
        population: 1000,
        kpis: vec![0],
        delegation: HashMap::new(),
        proportion_of_adversarials: 0.1,
        timesteps: 300,
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
            // let _prediction = self.simulate_prediction_market();
            // self.simulate_delegation();
            let voting_results = self.simulate_voting();
            self.simulate_world(&voting_results);
        }
    }

    fn simulate_world(&mut self, voting_results: &HashMap<String, usize>) {
        // voting_result.sort();
        let winner = voting_results.iter().max().unwrap();
        // let winner = voting_result[voting_result.len() - 1];
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

    fn simulate_prediction_market(&self) -> &usize {
        todo!()
        // &self.kpis.get(self.timestep)
    }

    fn simulate_delegation(&self) {
        todo!()
    }

    fn simulate_voting(&self) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        map.insert("Mixed".to_string(), 100);
        map.insert("Raddish".to_string(), 200);
        map.insert("Potato".to_string(), 300);
        map
    }
}

fn simulate_voting(population: usize) -> [usize; 3] {
    let preferences: Arc<Vec<[u8; 3]>> = Arc::new(preference_loading::every_nth(population, 4));
    let preferences_clone_1 = Arc::clone(&preferences);
    let preferences_clone_2 = Arc::clone(&preferences);
    let preferences_clone_3 = Arc::clone(&preferences);

    let mut vote_plurality = [0; 3];
    let mut vote_score = [0; 3];
    let mut vote_quadratic = [0; 3];

    let h1 = thread::spawn(move || {
        vote_plurality = plurality_voting(&preferences_clone_1);
        println!("{vote_plurality:?}");
        vote_plurality
    });

    let h2 = thread::spawn(move || {
        vote_score = score_voting(&preferences_clone_2);
        println!("{vote_score:?}");
        vote_score
    });

    let h3 = thread::spawn(move || {
        vote_quadratic = quadratic_voting(&preferences_clone_3);
        println!("{vote_quadratic:?}");
        vote_quadratic
    });

    h1.join().unwrap();
    let jh = h2.join().unwrap();
    h3.join().unwrap();

    jh
}
// Uses preferences to do plurality voting
// Also known as first past the post (FPTP)
fn plurality_voting(p: &Vec<[u8; 3]>) -> [usize; 3] {
    let mut sums = [0, 0, 0];
    for n in p {
        match n[0] {
            0 => sums[0] += 1,
            1 => sums[1] += 1,
            2 => sums[2] += 2,
            _ => {}
        }
    }
    return sums;
}

fn quadratic_voting(p: &Vec<[u8; 3]>) -> [usize; 3] {
    let mut sums = [0, 0, 0];

    for preferences in p {
        for i in 0..preferences.len() {
            sums[i] += (preferences.len() - i - 1).pow(2);
        }
    }
    sums
}

fn score_voting(preferences: &Vec<[u8; 3]>) -> [usize; 3] {
    let mut sums = [0, 0, 0];
    for preference in preferences {
        for i in 0..preference.len() {
            sums[preference[i] as usize] += preference.len() - i - 1;
        }
    }
    sums
}
