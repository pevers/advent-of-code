const NEIGHBORS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];
fn main() {
    let mut map = include_bytes!("../input")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
    let mut flashes = 0;
    (0..100).for_each(|_| {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                map[y][x] += 1;
            }
        }
        loop {
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] > b'9' {
                        map[y][x] = b'0';
                        flashes += 1;

                        NEIGHBORS
                            .iter()
                            .map(|(xx, yy)| {
                                ((x as isize + xx) as usize, (y as isize + yy) as usize)
                            })
                            .for_each(|(x, y)| match map.get(y).and_then(|l| l.get(x)) {
                                Some(b'0') => {}
                                Some(_) => map[y][x] += 1,
                                _ => {}
                            });
                    }
                }
            }

            if map.iter().all(|l| l.iter().all(|&c| c <= b'9')) {
                break;
            }
        }
    });
    println!("{}", flashes);
}
