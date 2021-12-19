fn main() {
    let content: Vec<(Vec<&str>, Vec<&str>)> = include_str!("../input")
        .split("\n")
        .map(|l| {
            let mut l = l.split("|");
            (
                l.next().unwrap().split_whitespace().collect(),
                l.next().unwrap().split_whitespace().collect(),
            )
        })
        .collect();
    let unique_length = vec![2, 3, 4, 7];
    println!(
        "{}",
        content
            .iter()
            .flat_map(|(_, o)| o)
            .filter(|o| unique_length.contains(&o.chars().count()))
            .count()
    );
}
