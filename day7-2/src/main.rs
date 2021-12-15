fn main() {
    let fuel: Vec<i32> = include_str!("../input")
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect();
    let min = *fuel.iter().min().unwrap();
    let max = *fuel.iter().max().unwrap();
    let mut costs = vec![-1; max as usize + 1];
    let optimal = (min..max)
        .map(|pos| {
            fuel.iter()
                .map(|f| {
                    // This is terrible slow, we should
                    // use Dynamic Programming (costs for every position)
                    // (1..=(*f - pos).abs()).sum::<i32>()

                    // Ok
                    let distance = (*f - pos).abs() as usize;
                    if costs[distance] == -1 {
                        costs[distance] = (1..=(*f - pos).abs()).sum::<i32>();
                    }
                    costs[distance]
                })
                .sum::<i32>()
        })
        .min()
        .unwrap();
    println!("{}", optimal);
}
