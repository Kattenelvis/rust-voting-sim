// fn preferenceAssignment() {
// }

fn main() {
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

    const N: usize = 10_usize.pow(8);

    let mut p: Vec<[i32; 3]> = Vec::with_capacity(N);
    for n in 0..N {
        p.push(if n % 4 == 0 { [0, 1, 2] } else { [1, 0, 2] });
    }

    // let p: Box<[[i32; 3]; N]> = Box::new(std::array::from_fn(|n| match n {
    //     _ if (n % 3 == 0) => [0, 1, 2],
    //     _ => [1, 0, 2],
    // }));

    // println!("{p:?}");

    // Plurality Voting (FPTP)
    let mut sums = [0, 0, 0];
    for n in p {
        match n[0] {
            0 => sums[0] += 1,
            1 => sums[1] += 1,
            2 => sums[2] += 2,
            _ => {}
        }

        // if n[0] == 0 {
        //     sums[0] += 1;
        // };
    }

    println!("{sums:?}");

    // let mut counts = [0, 0, 0];
    // for i in 0..10000 {}
    // println!("Hello, world!");
}
