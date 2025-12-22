fn main() {
    // Voting population size
    const N: usize = 10_usize.pow(7);

    let p = preference_loading(N);

    plurality_voting(p);
}

fn preference_loading(n: usize) -> Vec<[i32; 3]> {
    // Loading preference data into use
    // Assumes transitive and complete preferences (total ordering)
    let mut p: Vec<[i32; 3]> = Vec::with_capacity(n);
    for n in 0..n {
        p.push(if n % 4 == 0 { [0, 1, 2] } else { [1, 0, 2] });
    }

    return p;
}

fn plurality_voting(p: Vec<[i32; 3]>) {
    // Plurality Voting (FPTP)
    let mut sums = [0, 0, 0];
    for n in p {
        match n[0] {
            0 => sums[0] += 1,
            1 => sums[1] += 1,
            2 => sums[2] += 2,
            _ => {}
        }
    }

    println!("{sums:?}");
}

// fn preferenceAssignment() {
// }

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
