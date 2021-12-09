use std::fs;

fn main() {
    let lines: Vec<usize> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    let count = lines.len();
    let gamma = lines
        .iter()
        .fold([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], |accum, item| {
            [
                accum[0] + ((item >> 11) & 1),
                accum[1] + ((item >> 10) & 1),
                accum[2] + ((item >> 9) & 1),
                accum[3] + ((item >> 8) & 1),
                accum[4] + ((item >> 7) & 1),
                accum[5] + ((item >> 6) & 1),
                accum[6] + ((item >> 5) & 1),
                accum[7] + ((item >> 4) & 1),
                accum[8] + ((item >> 3) & 1),
                accum[9] + ((item >> 2) & 1),
                accum[10] + ((item >> 1) & 1),
                accum[11] + ((item >> 0) & 1),
            ]
        })
        .iter()
        .fold(0, |result, &bit| {
            (result << 1) ^ u16::from(bit > (count / 2))
        });
    // println!("{:#12b}", gamma);
    let epsilon = !(gamma | (15 << 12));
    println!("{}", gamma as f64 * epsilon as f64);
}
