use std::collections::HashMap;

fn main() {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();
    include_str!("../input").split("\n").for_each(|l| {
        let coords: Vec<u32> = l
            .split(" -> ")
            .flat_map(|c| c.split(",").map(|c| c.parse().unwrap()))
            .collect();
        let ((x, y), (xx, yy)) = (
            (coords[0].min(coords[2]), coords[1].min(coords[3])),
            (coords[0].max(coords[2]), coords[1].max(coords[3])),
        );
        if x == xx {
            (y..=yy).for_each(|y| *map.entry((x, y)).or_insert(0) += 1);
        }
        if y == yy {
            (x..=xx).for_each(|x| *map.entry((x, y)).or_insert(0) += 1);
        }
    });
    println!("{}", map.values().filter(|&v| *v >= 2).count());
}
