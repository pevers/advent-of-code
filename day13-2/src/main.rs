use std::collections::HashMap;

fn main() {
    let (input, fold) = include_str!("../input").split_once("\n\n").unwrap();
    let mut max_x = 0;
    let mut max_y = 0;
    let coords: HashMap<(usize, usize), u8> = input
        .split("\n")
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            max_x = x.max(max_x);
            max_y = y.max(max_y);
            ((x, y), 1)
        })
        .collect();
    max_x += 1;
    max_y += 1;
    let mut map: Vec<Vec<char>> = vec![vec!['.'; max_x]; max_y];
    for y in 0..max_y {
        for x in 0..max_x {
            if coords.get(&(x, y)).is_some() {
                map[y][x] = '#';
            }
        }
    }
    let map = fold.split("\n").fold(map.clone(), |accum, command| {
        let (command, coord) = command.split_once("=").unwrap();
        match command {
            "fold along y" => fold_y(&accum, coord.parse::<usize>().unwrap()),
            "fold along x" => fold_x(&accum, coord.parse::<usize>().unwrap()),
            _ => unreachable!(),
        }
    });

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", map[y][x]);
        }
        print!("\n");
    }
}

fn fold_y(map: &Vec<Vec<char>>, fold: usize) -> Vec<Vec<char>> {
    let folded_map: Vec<Vec<char>> = (0..map.len())
        .map(|y| {
            (0..map[0].len())
                .map(|x| {
                    let idy = fold as isize * 2 - y as isize;
                    if idy < 0 {
                        return map[y][x];
                    }
                    if map[y][x] != map[idy as usize][x] {
                        return '#';
                    }
                    map[y][x]
                })
                .collect()
        })
        .collect();
    folded_map[0..fold].to_vec()
}

fn fold_x(map: &Vec<Vec<char>>, fold: usize) -> Vec<Vec<char>> {
    (0..map.len())
        .map(|y| {
            (0..map[0].len())
                .map(|x| {
                    if x <= fold || x > fold * 2 {
                        return map[y][x];
                    }
                    let idx = x - (x - fold) * 2;
                    if map[y][x] != map[y][idx] {
                        return '#';
                    }
                    map[y][x]
                })
                .collect()
        })
        .map(|l: Vec<_>| l[fold + 1..].to_vec())
        .collect()
}
