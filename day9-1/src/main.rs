use std::collections::HashMap;

// Works, but ugly, we should be able 
// to make it smaller by being smarter with input reading
// and not creating a HashMap

fn main() {
    let content = include_str!("../input");
    let height = content.split("\n").count();
    let width = content.split("\n").next().unwrap().len();
    let mut map: HashMap<(isize, isize), u32> = HashMap::new();
    content.split("\n").enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            map.insert((x as isize, y as isize), c.to_digit(10).unwrap());
        })
    });
    let mut depths: Vec<u32> = vec![];
    for y in 0..height as isize {
        for x in 0..width as isize {
            let depth = map.get(&(x, y)).unwrap();
            if vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .filter_map(|c| map.get(c))
                .all(|d| depth < d)
            {
                depths.push(*depth + 1);
            }
        }
    }
    println!("{}", depths.iter().sum::<u32>());
}
