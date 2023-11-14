use std::cmp::Ordering;
use std::fs;

/// Our numbers look like 000001100010
/// We havee 1000 or them or so, and we want to figure out,
/// for each bit position across the numbers, what is the most
/// or least value (0 or 1).

fn get_parsed_numbers() -> Vec<u32> {
    fs::read_to_string("src/data/day_3.txt")
        .expect("couldn't read the file")
        .lines()
        .map(|bitstring| u32::from_str_radix(bitstring, 2).expect("parsing should be fine"))
        .collect()
}

fn get_most_common_numbers() -> Vec<&'static str> {
    let data = get_parsed_numbers();

    (0..12)
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
        .collect()
}

pub fn part_one() -> u32 {
    let most_common_numbers = get_most_common_numbers();

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

// PART TWO!
//
// Now we need to do something a bit more complicated. We want do drill down based on looking at
// the values in the input data and filtering in or out entries which, for the entries remaining in
// our input, have the most or least common bit. This is how the problem states it:
//
// Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in
// your diagnostic report - finding them is the tricky part. Both values are located using a
// similar process that involves filtering out values until only one remains. Before searching for
// either rating value, start with the full list of binary numbers from your diagnostic report and
// consider just the first bit of those numbers. Then:
//
// Keep only numbers selected by the bit criteria for the type of rating value for which you are
// searching. Discard numbers which do not match the bit criteria. If you only have one number
// left, stop; this is the rating value for which you are searching. Otherwise, repeat the process,
// considering the next bit to the right.
//
// The bit criteria depends on which type of rating value you want to find:
//
// To find oxygen generator rating, determine the most common value (0 or 1) in the current bit
// position, and keep only numbers with that bit in that position. If 0 and 1 are equally common,
// keep values with a 1 in the position being considered.
//
// To find CO2 scrubber rating, determine
// the least common value (0 or 1) in the current bit position, and keep only numbers with that bit
// in that position. If 0 and 1 are equally common, keep values with a 0 in the position being
// considered.

#[derive(Copy, Clone, Debug)]
struct BitCount {
    zero: usize,
    one: usize,
}

impl BitCount {
    fn new(bits: Vec<&str>) -> BitCount {
        BitCount {
            zero: bits.iter().filter(|bit| **bit == "0").count(),
            one: bits.iter().filter(|bit| **bit == "1").count(),
        }
    }

    /// Return the value which indicates we should keep an
    /// entry in the list
    fn keep_val(self, most_common: bool) -> &'static str {
        match self.zero.cmp(&self.one) {
            Ordering::Less => match most_common {
                true => "1",
                false => "0",
            },
            Ordering::Greater => match most_common {
                true => "0",
                false => "1",
            },
            Ordering::Equal => match most_common {
                true => "1",
                false => "0",
            },
        }
    }
}

fn filter_data(readings: Vec<&str>, index: usize, most_common: bool) -> &str {
    if readings.len() == 1 {
        readings[0]
    } else {
        let bit_count = BitCount::new(
            readings
                .iter()
                .map(|&reading| {
                    let split: Vec<&str> = reading.split("").collect();
                    let character = &split[index + 1];
                    *character
                })
                .collect(),
        );

        let filtered_data: Vec<&str> = readings
            .into_iter()
            .filter(|&reading| {
                let split: Vec<&str> = reading.split("").collect::<Vec<&str>>();
                let character = &split[index + 1];
                *character == bit_count.keep_val(most_common)
            })
            .collect();

        match filtered_data.len() {
            1 => &filtered_data[0],
            _ => filter_data(filtered_data, index + 1, most_common),
        }
    }
}

pub fn part_two() -> u32 {
    let raw_data = fs::read_to_string("src/data/day_3.txt").expect("couldn't read the file");
    let data: Vec<&str> = raw_data.lines().collect();

    let oxygen_generator_rating = u32::from_str_radix(filter_data(data.clone(), 0, true), 2)
        .expect("oxygen should calculate");

    let co2_scrubber_rating =
        u32::from_str_radix(filter_data(data.clone(), 0, false), 2).expect("CO2 should calculate");

    oxygen_generator_rating * co2_scrubber_rating
}
