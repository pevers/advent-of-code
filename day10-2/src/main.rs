use std::collections::{HashMap, VecDeque};

fn main() {
    let CHUNKS = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
    let POINTS: HashMap<_, _> = HashMap::from_iter([
        (')', 1 as u128),
        (']', 2 as u128),
        ('}', 3 as u128),
        ('>', 4 as u128),
    ]);
    let content = include_str!("../input").split("\n");
    let mut valid: Vec<_> = content
        .filter_map(|l| {
            let mut tokens: VecDeque<char> = VecDeque::new();
            for c in l.chars() {
                let chunk = CHUNKS
                    .iter()
                    .find(|chunk| chunk.0 == c || chunk.1 == c)
                    .unwrap();
                if c == chunk.0 {
                    // Opening token
                    tokens.push_back(c);
                } else {
                    // Closing token
                    if tokens.len() == 0 || tokens.pop_back().unwrap() != chunk.0 {
                        return None;
                    }
                }
            }
            // Finish it
            Some(tokens.iter().rev().fold(0, |acc, t| {
                acc * 5
                    + CHUNKS
                        .iter()
                        .find_map(|(o, c)| {
                            if o == t {
                                Some(POINTS.get(&c).unwrap())
                            } else {
                                None
                            }
                        })
                        .unwrap()
            }))
        })
        .collect();
    valid.sort();
    println!("{:?}", valid[valid.len() / 2]);
}
