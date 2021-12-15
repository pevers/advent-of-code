fn main() {
    let ticks: Vec<i8> = include_str!("../input")
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();
    let fish = (0..80)
        .fold(ticks.clone(), |t, _| {
            t.iter()
                .flat_map(|t| {
                    if t - 1 < 0 {
                        return vec![6, 8];
                    }
                    vec![t - 1]
                })
                .collect()
        })
        .iter()
        .count();
    println!("{}", fish);
}
