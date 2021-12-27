#![feature(int_roundings)]

// I first tried to do it building
// a balanced graph but that didn't really work out

type Snail = (u8, i8);

fn parse(input: &str) -> Vec<Snail> {
    let mut depth = 0;
    let mut result = Vec::new();
    input.as_bytes().iter().for_each(|&c| match c {
        b'[' => {
            depth = depth + 1;
        }
        b']' => {
            depth = depth - 1;
        }
        b',' => {}
        _ => {
            result.push((depth, (c - b'0') as i8));
        }
    });
    result
}

fn explode(snail: &mut Vec<Snail>) {
    let mut i = 0;
    while i < snail.len() {
        if snail[i].0 == 5 {
            if i > 0 && snail.get(i - 1).is_some() {
                snail[i - 1].1 += snail[i].1;
            }
            if i < snail.len() - 2 && snail.get(i + 2).is_some() {
                snail[i + 2].1 += snail[i + 1].1;
            }
            snail.drain(i..=i + 1);
            snail.insert(i, (4, 0));
            i = 0;
        } else {
            i += 1;
        }
    }
}

fn split(snail: &mut Vec<Snail>) {
    let mut i = 0;
    while i < snail.len() {
        if snail[i].1 >= 10 {
            let val = snail[i].1;
            let depth = snail[i].0;
            snail.insert(i, (depth + 1, val / 2));
            snail[i + 1].0 = depth + 1;
            snail[i + 1].1 = val.unstable_div_ceil(2);
            break;
        } else {
            i += 1;
        }
    }
}

fn magnitude(snail: &Vec<Snail>, depth: u8, i: &mut usize) -> u64 {
  let lhs = 3 * if snail[*i].0 == depth {
    *i += 1;
    snail[*i - 1].1 as u64
  } else {
    magnitude(snail, depth + 1, i)
  };
  let rhs = 2 * if snail[*i].0 == depth {
    *i += 1;
    snail[*i - 1].1 as u64
  } else {
    magnitude(snail, depth + 1, i)
  };
  lhs + rhs
}

fn reduce(snail: &mut Vec<Snail>) {
    loop {
        explode(snail);
        let len = snail.len();
        split(snail);
        if len == snail.len() {
            break;
        }
    }
}

fn main() {
    let content = include_str!("../input")
        .split("\n")
        .fold(vec![], |mut accum, curr| {
            let mut rhs: Vec<Snail> = parse(curr);
            if accum.len() > 0 {
              accum = accum.iter().map(|&(d,v)| (d+1, v)).collect();
              rhs = rhs.iter().map(|&(d, v)| (d+1, v)).collect();
            }
            accum.extend(rhs);
            reduce(&mut accum);
            accum
        });
    println!("{:?}", magnitude(&content, 1, &mut 0));
}

#[cfg(test)]
mod tests {
    use crate::{explode, parse, split};

    #[test]
    fn test_parse_snail() {
        let snail = parse("[[[[[9,8],1],2],3],4]");
        assert_eq!(vec![(5, 9), (5, 8), (4, 1), (3, 2), (2, 3), (1, 4)], snail);
    }

    #[test]
    fn test_explode() {
        let mut snail = parse("[[[[[9,8],1],2],3],4]");
        explode(&mut snail);
        assert_eq!(vec![(4, 0), (4, 9), (3, 2), (2, 3), (1, 4)], snail);
    }

    #[test]
    fn test_explode_two() {
        let mut snail = parse("[7,[6,[5,[4,[3,2]]]]]");
        explode(&mut snail);
        assert_eq!(vec![(1, 7), (2, 6), (3, 5), (4, 7), (4, 0)], snail);
    }

    #[test]
    fn test_split() {
        let mut snail = vec![
            (4, 0),
            (4, 7),
            (3, 4),
            (4, 7),
            (4, 8),
            (4, 0),
            (4, 13),
            (2, 1),
            (2, 1),
        ];
        split(&mut snail);
        assert_eq!(
            vec![
                (4, 0),
                (4, 7),
                (3, 4),
                (4, 7),
                (4, 8),
                (4, 0),
                (4, 6),
                (4, 7),
                (2, 1),
                (2, 1)
            ],
            snail
        );
    }
}
