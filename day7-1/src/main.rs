fn main() {
    let fuel: Vec<i32> = include_str!("../input")
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect();
    let min = *fuel.iter().min().unwrap();
    let max = *fuel.iter().max().unwrap();
    let optimal = (min..max)
        .map(|pos| {
            fuel.iter()
                .map(|f| (*f - pos).abs())
                .sum::<i32>()
        })
        .min()
        .unwrap();
    println!("{}", optimal);
}
