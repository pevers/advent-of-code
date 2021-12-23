fn main() {
    let input = "target area: x=269..292, y=-68..-44";
    let bounds: ((i32, i32), (i32, i32)) = input[13..]
        .split_once(", ")
        .and_then(|b| {
            let (x, xx) = b.0.split_once("..").unwrap();
            let (y, yy) = b.1.split_once("..").unwrap();
            Some((
                (x[2..].parse::<i32>().unwrap(), xx.parse::<i32>().unwrap()),
                (y[2..].parse::<i32>().unwrap(), yy.parse::<i32>().unwrap()),
            ))
        })
        .unwrap();
    
    // vy = start velocity in the y direction
    // vy will cross the x axis again with a -vy velocity
    // so -vy has to be less or equal than the most bottom y value
    // vx has to be less than max(x)?
    let count = (1..=bounds.0.1)
        .flat_map(|vx| {
            let range = bounds.1.0.abs();
            (-range..=range).filter(move |&vy| fire((vx,vy), &bounds))
        })
        .count();
    println!("{}", count);
}

/// Fire to location bound with a velocity
/// Returns true if it hits the target, otherwise false
fn fire(vel: (i32, i32), bounds: &((i32, i32),(i32,i32))) -> bool {
    let (mut vx, mut vy) = vel;
    let (mut x, mut y) = (0,0);
    loop {
        x += vx;
        y += vy;
        if vx == 0 && x < bounds.0.0 || x > bounds.0.1 || y < bounds.1.0 {
            return false;
        } 
        if x >= bounds.0.0 && x <= bounds.0.1 && y >= bounds.1.0 && y <= bounds.1.1 {
            return true;
        }
        vx = (vx - 1).max(0);
        vy = vy - 1;
    }
}
