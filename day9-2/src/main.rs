// Partly taken: https://github.com/timvisee/advent-of-code-2021/blob/master/day09a/src/main.rs

pub fn main() {
    type Coord = ((isize, isize), u8);
    let map = include_bytes!("../input")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();
    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut depths: Vec<Coord> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if neighbors.iter().all(|&(xx, yy)| {
                map.get((y as isize + yy) as usize)
                    .and_then(|l| l.get((x as isize + xx) as usize))
                    .map(|n| cell < n)
                    .unwrap_or(true)
            }) {
                depths.push(((x as isize, y as isize), cell - b'0'));
            }
        }
    }

    // We now have all depth points, recursively
    // iterate over all neighbours having the same
    // or more height until you hit 9

    let find_basin_inner = |map: &Vec<&[u8]>, ((x, y), depth): Coord| -> Vec<Coord> {
        neighbors
            .iter()
            .filter_map(move |&(xx, yy)| {
                map.get((y as isize + yy) as usize)
                    .and_then(|l| l.get((x as isize + xx) as usize))
                    .map(|n| ((x + xx as isize, y + yy as isize), n - b'0'))
            })
            .filter(|&(_, n)| depth < n && n != 9)
            .collect()
    };

    let find_basin = |depths: Coord| -> u32 {
        let mut basins = find_basin_inner(&map, depths);
        let mut total = 1;
        while !basins.is_empty() {
            total += basins.len() as u32;
            basins = basins
                .iter()
                .flat_map(|&b| find_basin_inner(&map, b))
                .collect();
        }
        total
    };

    let mut basins: Vec<u32> = depths.iter().map(|b| find_basin(*b)).collect();
    basins.sort_by(|a, b| b.cmp(a));
    println!("{:?}", basins[0] * basins[1] * basins[2]);
}
