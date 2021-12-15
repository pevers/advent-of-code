fn main() {
    let mut counts = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    include_str!("../input")
        .split(",")
        .map(|c| c.parse().unwrap())
        .for_each(|t: usize| {
            counts[t] += 1.0
        });
    (0..256)
        .for_each(|_| {
            let spawn = counts[0];
            for tick in 0..8 {
                counts[tick] = counts[tick+1]
            }
            counts[6] += spawn;
            counts[8] = spawn;
        });
    println!("{}", counts.iter().sum::<f64>());
}
