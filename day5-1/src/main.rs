use std::collections::HashMap;

fn main() {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();
    include_str!("../input").split("\n").for_each(|l| {
        let coords: Vec<u32> = l
            .split(" -> ")
            .flat_map(|c| c.split(",").map(|c| c.parse().unwrap()))
            .collect();
        // Vertical line
        if coords[0] == coords[2] {
            let min = coords[1].min(coords[3]);
            let max = coords[1].max(coords[3]);
            (min..max + 1).for_each(|y| {
                *map.entry((coords[0], y)).or_insert(0) += 1;
            });
        } else if coords[1] == coords[3] {
            let min = coords[0].min(coords[2]);
            let max = coords[0].max(coords[2]);
            (min..max + 1).for_each(|x| {
                *map.entry((x, coords[1])).or_insert(0) += 1;
            });
        }
    });
    println!("{}", map.values().filter(|&v| *v >= 2).count());
}
