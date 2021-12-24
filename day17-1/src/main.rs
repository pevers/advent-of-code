fn main() {
    let input = "target area: x=269..292, y=-68..-44";
    let bounds: (i32, i32) = input[13..]
        .split_once(", ")
        .and_then(|(_,by)| {
            let (y, yy) = by.split_once("..").unwrap();
            Some((y[2..].parse::<i32>().unwrap(), yy.parse::<i32>().unwrap()))
        })
        .unwrap();
    // Solve n(n+1)/2 where n = -(min(vy)) - 1
    let n = -bounds.0.min(bounds.1) - 1;
    println!("{}", (n*(n+1))/2);
}
