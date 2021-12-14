use std::collections::HashMap;

fn main() {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    include_str!("../input").split("\n").for_each(|l| {
        let coords: Vec<i32> = l
            .split(" -> ")
            .flat_map(|c| c.split(",").map(|c| c.parse().unwrap()))
            .collect();
        let ((x, y), (xx, yy)) = (
            (coords[0].min(coords[2]), coords[1].min(coords[3])),
            (coords[0].max(coords[2]), coords[1].max(coords[3])),
        );
        if x == xx {
            (y..=yy).for_each(|y| *map.entry((x, y)).or_insert(0) += 1);
        } else if y == yy {
            (x..=xx).for_each(|x| *map.entry((x, y)).or_insert(0) += 1);
        } else {
            // Diagonal
            let dx = coords[0] - coords[2];
            let dy = coords[1] - coords[3];
            if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                (x..=xx)
                    .zip(y..=yy)
                    .for_each(|(x, y)| *map.entry((x, y)).or_insert(0) += 1);
            } else if (dx < 0 && dy > 0) || (dx > 0 && dy < 0) {
                (x..=xx)
                    .zip((y..=yy).rev())
                    .for_each(|(x, y)| *map.entry((x, y)).or_insert(0) += 1);
            }
        }
    });
    println!("{}", map.values().filter(|&v| *v >= 2).count());
}
