use pathfinding::prelude::dijkstra;

const NEIGHBORS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
fn main() {
    let map: Vec<Vec<u32>> = include_str!("../input")
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as u32).collect())
        .collect();

    // Generate new map
    let width = map.len();
    let height = map[0].len();
    let mut resized_map: Vec<Vec<u32>> = vec![vec![0; width * 5]; height * 5];
    for y in 0..(height * 5) {
        for x in 0..(width * 5) {
            let yy = y % height;
            let xx = x % height;
            let mut new_value = map[yy][xx] + (y / height + x / width) as u32;
            if new_value > 9 {
                new_value = new_value % 9;
                if new_value == 0 {
                    new_value = 1;
                }
            }
            resized_map[y][x] = new_value;
        }
    }

    let end = (
        (resized_map.len() - 1) as isize,
        (resized_map[0].len() - 1) as isize,
    );
    let route = dijkstra(
        &(0, 0),
        |(x, y)| {
            NEIGHBORS
                .iter()
                .map(|(xx, yy)| ((xx + x) as usize, (yy + y) as usize))
                .filter_map(|(x, y)| {
                    resized_map
                        .get(y)
                        .and_then(|l| l.get(x))
                        .map(|r| ((x as isize, y as isize), *r))
                })
                .collect::<Vec<_>>()
        },
        |&p| p == end,
    )
    .unwrap()
    .1;
    println!("{}", route);
}
