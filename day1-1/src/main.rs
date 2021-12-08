use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("input")
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<u32>>()
            .windows(2)
            .fold(0, |accum, item| {
                if item[1] > item[0] {
                    return accum + 1;
                }
                accum
            })
    );
}