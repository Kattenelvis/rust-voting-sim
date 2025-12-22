// fn preferenceAssignment() {
// }

fn main() {
    const N: i32 = 10_i32.pow(6);
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

    let p: [[i32; 3]; 10] = std::array::from_fn(|n| match n {
        val if (n % 3 == 0) => [0, 1, 2],
        _ => [1, 0, 2],
    });

    println!("{p:?}");

    // Voting

    // let mut counts = [0, 0, 0];
    // for i in 0..10000 {}
    // println!("Hello, world!");
}
