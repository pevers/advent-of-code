use std::fs;

// Some nice functional fu

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let draws: Vec<u32> = content
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    let mut boards: Vec<(u32, bool)> = content
        .lines()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(6)
        .flat_map(|l| {
            return l
                .iter()
                .skip(1)
                .flat_map(|l| l.split_whitespace().map(|c| (c.parse().unwrap(), false)))
                .collect::<Vec<(u32, bool)>>();
        })
        .collect();

    for draw in draws {
        boards.iter_mut().for_each(|(v, mark)| {
            if *v == draw {
                *mark = true;
            }
        });
        // Check if a player won
        let winning_boards: Vec<&[(u32, bool)]> = boards
            .chunks(25)
            .into_iter()
            .filter(|board| {
                let has_row = board
                    .chunks(5)
                    .any(|row| row.iter().all(|(v, mark)| *mark == true));
                let has_col = (0..5).any(|i| {
                    board
                        .iter()
                        .skip(i)
                        .step_by(5)
                        .all(|(v, mark)| *mark == true)
                });
                has_row || has_col
            })
            .collect();
        if winning_boards.len() >= 1 {
            // Just return the first one
            let unmarked_sum: u32 = winning_boards[0]
                .iter()
                .filter_map(|(v, mark)| {
                    if *mark == false {
                        Some(v.to_owned() as u32)
                    } else {
                        None
                    }
                })
                .sum();
            println!("{:?}", unmarked_sum * draw);
            break;
        }
    }
}
