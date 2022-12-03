#![feature(iter_array_chunks)]
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    // Part 1
    let priority_sum = input.split("\n").fold(0, |priority, rucksack| {
        let (comp_1, comp_2) = rucksack.split_at(rucksack.len() / 2);
        let item = get_shared_item(comp_1, comp_2);
        match item {
            Some(c) => priority + get_item_priority(c),
            None => priority,
        }
    });

    // Part 2
    let badge_sum = input.split("\n").array_chunks::<3>().fold(0, |priority, bags| {
        let badge_type = get_badge_type(bags[0], bags[1], bags[2]);
        match badge_type {
            Some(c) => priority + get_item_priority(c),
            None => priority
        }
    });

    println!("{}", priority_sum);
    println!("{}", badge_sum);
}

fn get_shared_item(comp_1: &str, comp_2: &str) -> Option<char> {
    let comp_1_set: HashSet<char> = comp_1.chars().collect();
    let comp_2_set: HashSet<char> = comp_2.chars().collect();
    comp_1_set.intersection(&comp_2_set).nth(0).cloned()
}

fn get_item_priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        item as u32 - 96
    } else {
        item as u32 - 38
    }
}

fn get_badge_type(bag_1: &str, bag_2: &str, bag_3: &str) -> Option<char> {
    let bag_1_set: HashSet<char> = bag_1.chars().collect();
    let bag_2_set: HashSet<char> = bag_2.chars().collect();
    let bag_3_set: HashSet<char> = bag_3.chars().collect();
    let i_1: HashSet<char> = bag_1_set.intersection(&bag_2_set).cloned().collect();
    i_1.intersection(&bag_3_set).nth(0).cloned()
}

