use code_timing_macros::time_function;
use std::sync::Arc;
use std::thread;

mod kpi;
mod preference_loading;

#[time_function]
fn main() {
    let n = kpi::kpi::main();
    println!("{n:?}");
    // Voting population size
    const N: u32 = 2 * 10_u32.pow(5);

    let preferences: Arc<Vec<[u8; 3]>> =
        Arc::new(preference_loading::preference_loading::every_nth(N, 4));
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
    h2.join().unwrap();
    h3.join().unwrap();
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
