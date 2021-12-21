use std::collections::HashMap;

fn main() {
    let (template, pairs) = include_str!("../input").split_once("\n\n").unwrap();
    let sequences: HashMap<[u8; 2], u8> = pairs
        .split("\n")
        .map(|p| {
            let (left, right) = p.split_once(" -> ").unwrap();
            ([left.as_bytes()[0], left.as_bytes()[1]], right.as_bytes()[0])
        })
        .collect();

    // Recursively solve this puzzle, by first trying the outer most then divide it by 2
    // Divide and Conquer!
    let mut cache:HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
    let sol = (0..10).fold(template.as_bytes().to_vec(),|acc, i| {
        println!("iteration {}", i);
        solve(&acc, &mut cache, &sequences)
    });
    let mut count:HashMap<u8, u128> = HashMap::new();
    sol.iter().for_each(|&c| {
        *count.entry(c).or_insert(0) += 1;
    });
    let most_common = count.values().max().unwrap();
    let least_common = count.values().min().unwrap();
    println!("{}", most_common - least_common);
}

fn solve(polymer: &[u8], cache: &mut HashMap<Vec<u8>, Vec<u8>>, sequences: &HashMap<[u8; 2], u8>) -> Vec<u8> {
    if let Some(c) = cache.get(polymer) {
        return c.to_owned();
    }
    if polymer.len() == 1 {
        return polymer.to_vec();
    }
    let (left, right) = polymer.split_at(polymer.len() / 2);
    let mut left = solve(left, cache, sequences);
    let right = solve(right, cache, sequences);

    // Stitch them together
    let c = sequences.get(&[left[left.len()-1], right[0]]).unwrap();
    left.push(*c);
    left.extend(right);
    cache.insert(polymer.to_owned(), left.to_owned());
    left
}
