use code_timing_macros::time_function;
use std::sync::Arc;
use std::thread;

#[time_function]
fn main() {
    // Voting population size
    const N: u32 = 10_u32.pow(8);

    let preferences: Arc<Vec<[u8; 3]>> = Arc::new(preference_loading(N));
    let preferences_clone_1 = Arc::clone(&preferences);
    let preferences_clone_2 = Arc::clone(&preferences);

    let mut vote_plurality = [42; 3];
    let mut vote_copeland = [42; 3];

    thread::spawn(move || {
        vote_plurality = plurality_voting(&preferences_clone_1);
        println!("{vote_plurality:?}");
    });

    thread::spawn(move || {
        vote_copeland = copeland_voting(&preferences_clone_2);
        println!("{vote_copeland:?}");
    });

    // handle.join().unwrap();
    // handle1.join().unwrap();
}

fn preference_loading(population: u32) -> Vec<[u8; 3]> {
    let pop = population as usize;
    // Loading preference data into use
    // Assumes transitive and complete preferences (total ordering)
    // TODO: Perhaps it's better to have key value pair from a preference to the number of holders
    // of that preference
    let mut preferences: Vec<[u8; 3]> = Vec::with_capacity(pop);
    for n in 0..pop {
        preferences.push(if n % 4 == 0 { [0, 1, 2] } else { [1, 0, 2] });
    }

    return preferences;
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

fn copeland_voting(preferences: &Vec<[u8; 3]>) -> [usize; 3] {
    let mut sums = [0, 0, 0];
    for preference in preferences {
        for i in 0..preference.len() {
            sums[preference[i] as usize] += preference.len() - i - 1;
        }
    }
    return sums;
}

// fn symmetric_borda_score(preferences: Vec<[usize; 3]>) -> [usize; 3] {}

// let candidates = ["a", "b", "c"];

// let preferences = [["a"], ["b"]];
// Loading preference data into use
// let mut pref: [str; 10000] = Vec::with_capacity(10000);

// let mut p = Vec::with_capacity(N);
// let mut p: Vec<_> = (0..N).collect();
// println!("{p:?}");
// for i in 0..N {
//     p.push(["a", "b", "c"]);
//     println!("{}", p[i]);
// }

// let p: Box<[[i32; 3]; N]> = Box::new(std::array::from_fn(|n| match n {
//     _ if (n % 3 == 0) => [0, 1, 2],
//     _ => [1, 0, 2],
// }));

// println!("{p:?}");
