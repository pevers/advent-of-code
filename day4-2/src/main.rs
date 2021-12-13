use std::fs;

// It ain't pretty but it works

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let draws: Vec<u32> = content
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    let mut boards: Vec<(bool, Vec<(u32, bool)>)> = content
        .lines()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|l| {
            let board = l
                .iter()
                .skip(1)
                .flat_map(|l| l.split_whitespace().map(|c| (c.parse().unwrap(), false)))
                .collect::<Vec<(u32, bool)>>();
            (false, board)
        })
        .collect();

    let mut finished_boards = 0;
    let board_count = boards.len();
    for draw in draws {
        for (won, board) in boards.iter_mut() {
            if *won {
                continue;
            }

            // Mark everything
            board.iter_mut().for_each(|(v, mark)| {
                if *v == draw {
                    *mark = true;
                }
            });

            // Check if this board won
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

            if has_row || has_col {
                *won = true;
                finished_boards += 1;
            }
            if finished_boards == board_count {
                let unmarked_sum: u32 = board
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
}
