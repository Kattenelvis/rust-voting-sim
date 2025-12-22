fn main() {
    // Voting population size
    const N: usize = 10_usize.pow(7);

    let p = preference_loading(N);

    // let vote = plurality_voting(p);
    let vote = copeland_voting(p);
    println!("{vote:?}");
}

fn preference_loading(n: usize) -> Vec<[usize; 3]> {
    // Loading preference data into use
    // Assumes transitive and complete preferences (total ordering)
    // TODO: Perhaps it's better to have key value pair from a preference to the number of holders
    // of that preference
    let mut p: Vec<[usize; 3]> = Vec::with_capacity(n);
    for n in 0..n {
        p.push(if n % 4 == 0 { [0, 1, 2] } else { [1, 0, 2] });
    }

    return p;
}

// Uses preferences to do plurality voting
// Also known as first past the post (FPTP)
fn plurality_voting(p: Vec<[i32; 3]>) -> [i32; 3] {
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

fn copeland_voting(preferences: Vec<[usize; 3]>) -> [usize; 3] {
    let mut sums = [0, 0, 0];
    for preference in preferences {
        for i in 0..preference.len() {
            sums[preference[i]] += preference.len() - i - 1;
        }
    }
    return sums;
}

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
