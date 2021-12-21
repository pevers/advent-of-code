use std::collections::HashMap;

fn main() {
    let (template, pairs) = include_str!("../input").split_once("\n\n").unwrap();
    let rules: HashMap<String, (String, String)> = pairs
        .split("\n")
        .map(|p| {
            let (left, right) = p.split_once(" -> ").unwrap();
            let l = left.as_bytes();
            let r = right.as_bytes();
            (
                left.to_string(),
                (
                    (String::from_utf8(vec![l[0], r[0]]).unwrap()),
                    String::from_utf8(vec![r[0], l[1]]).unwrap(),
                ),
            )
        })
        .collect();
    let mut pairs: HashMap<String, u128> = template
        .chars()
        .zip(template.chars().skip(1))
        .map(|(a, b)| (format!("{}{}", a, b), 1))
        .collect();
    for i in 0..40 {
        let mut new_counter = HashMap::new();
        pairs.iter().for_each(|(k, v)| {
            *new_counter
                .entry(rules.get(k).unwrap().0.to_owned())
                .or_insert(0) += v;
            *new_counter
                .entry(rules.get(k).unwrap().1.to_owned())
                .or_insert(0) += v;
        });
        pairs = new_counter;
    }
    let mut letters: HashMap<u8, u128> = HashMap::new();
    pairs.iter().for_each(|(k, v)| {
        *letters.entry(k.as_bytes()[0]).or_insert(0) += v;
    });
    *letters
        .entry(template.as_bytes()[template.as_bytes().len() - 1])
        .or_insert(0) += 1;
    println!(
        "{}",
        letters.values().max().unwrap() - letters.values().min().unwrap()
    );
}
