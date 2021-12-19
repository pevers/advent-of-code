#![feature(slice_group_by)]
// Taken from: https://github.com/timvisee/advent-of-code-2021/blob/master/day08b/src/main.rs#L4
// But then slightly different

fn main() {
    let answer = include_str!("../input")
        .split("\n")
        .map(|line| {
            let mut part = line.splitn(2, "|");
            let mut input = part.next().unwrap().split_whitespace();
            let one = input.clone().find(|d| d.len() == 2).unwrap();
            let four = input.find(|d| d.len() == 4).unwrap();
            part.next()
                .unwrap()
                .split_whitespace()
                .map(|d| match d.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    len => match (
                        len,
                        d.chars().filter(|&c| one.contains(c)).count(),
                        d.chars().filter(|&c| four.contains(c)).count(),
                    ) {
                        (5, 1, 3) => 5,
                        (5, 2, 3) => 3,
                        (5, _, 2) => 2,
                        (6, 1, _) => 6,
                        (6, _, 3) => 0,
                        (6, _, 4) => 9,
                        _ => unreachable!(),
                    },
                })
                .enumerate()
                .fold(0, |acc, (i, n)| acc + n * 10_u32.pow(3 - i as u32))
        })
        .sum::<u32>();
    println!("{}", answer);
}
