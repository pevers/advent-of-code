use std::fs;

fn main() {
    let (aim, horizontal, depth) = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let chunks: Vec<&str> = line.split_whitespace().collect();
            (chunks[0].to_owned(), chunks[1].parse().unwrap())
        })
        .collect::<Vec<(String, u32)>>()
        .iter()
        // aim, horizontal, depth
        .fold((0, 0, 0), |accum, (command, input)| {
            return match command.as_str() {
                "forward" => (accum.0, accum.1 + input, accum.2 + accum.0 * input),
                "down" => (accum.0 + input, accum.1, accum.2),
                "up" => (accum.0 - input, accum.1, accum.2),
                _ => panic!("invalid command"),
            };
        });
    println!("{}", horizontal * depth);
}
