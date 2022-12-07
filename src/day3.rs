use crate::lib::get_file_lines;
use anyhow::Result;
use std::collections::hash_set::HashSet;

/// One Elf has the important job of loading all of the rucksacks with supplies for the jungle
/// journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few
/// items now need to be rearranged.
///
/// Each rucksack has two large compartments. All items of a given type are meant to go into
/// exactly one of the two compartments. The Elf that did the packing failed to follow this rule
/// for exactly one item type per rucksack.
///
/// The Elves have made a list of all of the items currently in each rucksack (your puzzle input),
/// but they need your help finding the errors. Every item type is identified by a single lowercase
/// or uppercase letter (that is, a and A refer to different types of items).
///
/// The list of items for each rucksack is given as characters all on a single line. A given
/// rucksack always has the same number of items in each of its two compartments, so the first half
/// of the characters represent items in the first compartment, while the second half of the
/// characters represent items in the second compartment.
///
/// For example, suppose you have the following list of contents from six rucksacks:
///
/// vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw
///
/// The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first
/// compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items
/// hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
///
/// The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item
/// type that appears in both compartments is uppercase L.
///
/// The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is
/// uppercase P. The fourth rucksack's compartments only share item type v. The fifth rucksack's
/// compartments only share item type t. The sixth rucksack's compartments only share item type s.
///
/// To help prioritize item rearrangement, every item type can be converted to a priority:
///
/// Lowercase item types a through z have priorities 1 through 26.
///
/// Uppercase item types A through Z have priorities 27 through 52.
///
/// In the above example, the priority of the item type that appears in both compartments of each
/// rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
///
/// Find the item type that appears in both compartments of each rucksack. What is the sum of the
/// priorities of those item types?

/// an Item is represented by a single character
type Item = char;

/// A rucksack has two compartments, with items in them
#[derive(Debug)]
struct Rucksack {
    a: HashSet<Item>,
    b: HashSet<Item>,
}

impl Rucksack {
    pub fn new(items: &str) -> Rucksack {
        let compartment_one = &items[0..(items.len() / 2)];
        let compartment_two = &items[(items.len() / 2)..items.len()];

        Rucksack {
            a: HashSet::from_iter(compartment_one.chars()),
            b: HashSet::from_iter(compartment_two.chars()),
        }
    }

    pub fn get_shared_item(&self) -> Item {
        let mut intersection = self.a.intersection(&self.b);
        intersection
            .next()
            .expect("we should find a shared item in every backpack")
            .to_owned()
    }
}

fn priority_for_item(item: Item) -> u32 {
    let ascii = item as u32;

    if (65..=90).contains(&ascii) {
        // uppercase!
        // this offset looks magical, but basically we need to turn 'A' == 65 into 27
        // 65 - x = 27 -> x = -(27 - 65) -> x = 38
        ascii - 38
    } else {
        // must be lowercase
        ascii - 96
    }
}

pub fn solution() -> Result<()> {
    let lines = get_file_lines("data/day_3.txt")?;

    let sum = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let rucksack = Rucksack::new(line);
            priority_for_item(rucksack.get_shared_item())
        })
        .sum::<u32>();

    println!("day 3, part 1: sum is {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_example() {
        let lines = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let sum = lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let rucksack = Rucksack::new(line);
                let p = priority_for_item(rucksack.get_shared_item());
                println!("priority for {:?} is {}", &rucksack, p);
                p
            })
            .sum::<u32>();
        assert!(sum == 157);
    }
}
