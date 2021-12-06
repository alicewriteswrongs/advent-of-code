use std::fs;

/// Our numbers look like 000001100010
/// We havee 1000 or them or so, and we want to figure out,
/// for each bit position across the numbers, what is the most
/// or least value (0 or 1).

fn get_data() -> Vec<u32> {
    fs::read_to_string("src/data/day_3.txt")
        .expect("couldn't read the file")
        .lines()
        .map(|bitstring| u32::from_str_radix(bitstring, 2).expect("parsing should be fine"))
        .collect()
}

fn get_test_data() -> Vec<u32> {
    let test = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    test.lines()
        .map(|bitstring| u32::from_str_radix(bitstring, 2).expect("parsing should be fine"))
        .collect()
}

pub fn part_one() -> u32 {
    let data = get_data();

    let most_common_numbers: Vec<&str> = (0..12)
        .rev()
        .map(|right_shift| {
            let sum: u32 = data.iter().fold(0, |acc, number| {
                let shifted_number = number >> right_shift;
                let masked_shifted_number = shifted_number & 1;
                acc + masked_shifted_number
            });

            match sum <= 500 {
                true => "0",
                false => "1",
            }
        })
        .collect();

    let gamma = u32::from_str_radix(&most_common_numbers.join(""), 2).expect("this should work");

    let epsilon = u32::from_str_radix(
        &most_common_numbers
            .iter()
            .map(|num| {
                if *num == "0" {
                    return "1";
                }
                return "0";
            })
            .collect::<Vec<&str>>()
            .join(""),
        2,
    )
    .expect("this should work too!");

    return gamma * epsilon;
}
