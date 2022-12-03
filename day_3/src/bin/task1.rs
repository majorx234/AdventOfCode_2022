use aoc_lib::read_arg_file;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();

    let mut sum = 0;
    for rucksack in reader.lines() {
        let mut rucksack1_map: HashMap<char, u32> = HashMap::new();
        if let Ok(rucksack_content) = rucksack {
            let rucksack_size = rucksack_content.len();
            if rucksack_size >= 2 {
                let rucksack1_size = (rucksack_size / 2) as usize;
                let rucksack1 = &rucksack_content[0..rucksack1_size];
                let rucksack2 = &rucksack_content[rucksack1_size..rucksack_size];

                for item in rucksack1.chars() {
                    let mut value: u32 = 0;

                    if item.is_lowercase() {
                        value = item as u32 - 96;
                    } else if item.is_uppercase() {
                        value = item as u32 - 38;
                    }
                    rucksack1_map.insert(item, value);
                }
                // find dublicate in 2nd part of rucksack
                for item in rucksack2.chars() {
                    if let Some(value) = rucksack1_map.get(&item) {
                        sum += value;
                        break;
                    }
                }
            }
        }
    }
    println!("First: {}", sum);
}
