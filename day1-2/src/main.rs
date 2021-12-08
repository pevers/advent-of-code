use std::fs;

fn main() {
    let depths = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u32>>();

    let answer = depths
        .windows(3)
        .zip(depths.windows(3).skip(1))
        .fold(0, |accum, (a, b)| {
            if b[0] + b[1] + b[2] > a[0] + a[1] + a[2] {
                return accum + 1;
            }
            accum
        });
    println!("{}", answer);
}
