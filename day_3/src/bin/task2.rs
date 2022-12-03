use aoc_lib::read_arg_file;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();

    let mut sum = 0;
    let mut triplet_state = 0;
    let mut rucksack1_map: HashMap<char, u32> = HashMap::new();
    let mut rucksack2_map: HashMap<char, u32> = HashMap::new();
    for rucksack in reader.lines() {
        if let Ok(rucksack_content) = rucksack {
            let rucksack_size = rucksack_content.len();
            if triplet_state == 0 {
                for item in rucksack_content.chars() {
                    let mut value: u32 = 0;

                    if item.is_lowercase() {
                        value = item as u32 - 96;
                    } else if item.is_uppercase() {
                        value = item as u32 - 38;
                    }
                    rucksack1_map.insert(item, value);
                }
                triplet_state = 1;
            } else if triplet_state == 1 {
                // find dublicate in 2nd elf's of rucksack
                for item in rucksack_content.chars() {
                    if let Some(value) = rucksack1_map.get(&item) {
                        rucksack2_map.insert(item, *value);
                    }
                }
                triplet_state = 2;
            } else {
                for item in rucksack_content.chars() {
                    if let Some(value) = rucksack2_map.get(&item) {
                        sum += value;
                        rucksack1_map.clear();
                        rucksack2_map.clear();
                        triplet_state = 0;
                        break;
                    }
                }
            }
        }
    }
    println!("First: {}", sum);
}
