#![feature(format_args_capture)]
use std::collections::{HashMap, VecDeque};

fn main() {
    let CHUNKS = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
    let POINTS: HashMap<_, _> =
        HashMap::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let content = include_str!("../input").split("\n");
    let total = content
        .map(|l| {
            let mut tokens:VecDeque<char> = VecDeque::new();
            let mut score = 0;
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
                    if tokens.len() == 0 || tokens.pop_back().unwrap() != chunk.0{
                        score = *POINTS.get(&c).unwrap();
                        break;
                    }
                }
            }
            score
        })
        .sum::<u32>();
    println!("{total}");
}
