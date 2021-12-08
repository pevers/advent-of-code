use std::fs;

fn main() {
    let (horizontal, depth) = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let chunks: Vec<&str> = line.split_whitespace().collect();
            (chunks[0].to_owned(), chunks[1].parse().unwrap())
        })
        .collect::<Vec<(String, u32)>>()
        .iter()
        .fold((0, 0), |accum, (command, input)| {
            return match command.as_str() {
                "forward" => (accum.0 + input, accum.1),
                "down" => (accum.0, accum.1 + input),
                "up" => (accum.0, accum.1 - input),
                _ => panic!("invalid command"),
            };
        });
    println!("{}", horizontal * depth);
}
