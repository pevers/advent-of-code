use std::fs;

fn main() {
    let mut horizontal = 0;
    let mut depth = 0;
    fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let chunks: Vec<&str> = line.split_whitespace().collect();
            (chunks[0].to_owned(), chunks[1].parse().unwrap())
        })
        .collect::<Vec<(String, u32)>>()
        .iter()
        .for_each(|(command, input)| match command.as_str() {
            "forward" => horizontal += input,
            "down" => depth += input,
            "up" => depth -= input,
            _ => panic!("invalid command"),
        });
    println!("{}", horizontal * depth);
}
