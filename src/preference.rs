use std::sync::Arc;
use std::thread;

pub fn simulate_voting(population: usize) -> [usize; 3] {
    let preferences: Arc<Vec<[u8; 3]>> =
        Arc::new(crate::preference_loading::every_nth(population, 4));
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
