use pathfinding::prelude::dijkstra;

const NEIGHBORS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    let map: Vec<Vec<u32>> = include_str!("../input")
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as u32).collect())
        .collect();
    let end = ((map.len() - 1) as isize, (map[0].len() - 1) as isize);
    let route = dijkstra(
        &(0, 0),
        |(x, y)| {
            NEIGHBORS
                .iter()
                .map(|(xx, yy)| ((xx + x) as usize, (yy + y) as usize))
                .filter_map(|(x, y)| {
                    map.get(y)
                        .and_then(|l| l.get(x))
                        .map(|r| ((x as isize, y as isize), *r))
                })
                .collect::<Vec<_>>()
        },
        |&p| p == end,
    )
    .unwrap().1;
    println!("{}", route);
}
