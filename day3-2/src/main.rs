use std::fs;

// Ugly solution. Don't know how to cut this down functionaly

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let values: Vec<usize> = content
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    let line_length = content.lines().next().unwrap().len();
    let (mut oxygen_generator_rating_values, mut co2_scrubber_rating_values) =
        filter_rating(&values, line_length - 1);
    for bit in (0..line_length - 1).rev() {
        if oxygen_generator_rating_values.len() != 1 {
            oxygen_generator_rating_values = filter_rating(&oxygen_generator_rating_values, bit).0;
        }
        if co2_scrubber_rating_values.len() != 1 {
            co2_scrubber_rating_values = filter_rating(&co2_scrubber_rating_values, bit).1;
        }
    }
    let (oxygen_generator_rating, co2_scrubber_rating) =
        calculate_rating(&oxygen_generator_rating_values, &co2_scrubber_rating_values);
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
}

/// Filter ratings and return the (oxygen_generator_rating, co2_scrubber_rating) values
fn filter_rating(input: &Vec<usize>, bit_position: usize) -> (Vec<usize>, Vec<usize>) {
    let zeros = filter(&input, bit_position, 0);
    let ones = filter(&input, bit_position, 1);
    if ones.len() >= zeros.len() {
        return (ones, zeros);
    }
    (zeros, ones)
}

/// Filter out the values in input for a given bit_position
/// having value 0 or 1
fn filter(input: &Vec<usize>, bit_position: usize, value: usize) -> Vec<usize> {
    input
        .iter()
        .filter(|item| (((*item.to_owned() >> bit_position) & 1) ^ value) == 0)
        .map(|item| item.to_owned())
        .collect::<Vec<usize>>()
}

/// Return the "Oxygen Generator Rating" and the "CO2 Scrubber Rating" or -1
/// if not known
fn calculate_rating(
    oxygen_generator_rating_values: &Vec<usize>,
    co2_scrubber_rating_values: &Vec<usize>,
) -> (i32, i32) {
    let mut oxygen = -1;
    let mut co2 = -1;
    if oxygen_generator_rating_values.len() == 1 {
        oxygen = oxygen_generator_rating_values[0] as i32;
    }
    if co2_scrubber_rating_values.len() == 1 {
        co2 = co2_scrubber_rating_values[0] as i32;
    }
    (oxygen, co2)
}