use aoc_lib::read_arg_file;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();

    let mut rules1 = HashMap::new();
    rules1.insert("A X".to_string(), 4);
    rules1.insert("A Y".to_string(), 8);
    rules1.insert("A Z".to_string(), 3);
    rules1.insert("B X".to_string(), 1);
    rules1.insert("B Y".to_string(), 5);
    rules1.insert("B Z".to_string(), 9);
    rules1.insert("C X".to_string(), 7);
    rules1.insert("C Y".to_string(), 2);
    rules1.insert("C Z".to_string(), 6);

    let mut rules2 = HashMap::new();
    rules2.insert("A X".to_string(), 3);
    rules2.insert("A Y".to_string(), 4);
    rules2.insert("A Z".to_string(), 8);
    rules2.insert("B X".to_string(), 1);
    rules2.insert("B Y".to_string(), 5);
    rules2.insert("B Z".to_string(), 9);
    rules2.insert("C X".to_string(), 2);
    rules2.insert("C Y".to_string(), 6);
    rules2.insert("C Z".to_string(), 7);

    let sum1_rules1: (HashMap<String, u32>, u32) = (rules1, 0);
    let sum2_rules2: (HashMap<String, u32>, u32) = (rules2, 0);

    let sum_up = |acc, x: Result<String, _>| -> (HashMap<String, u32>, u32) {
        let (map, mut sum): (HashMap<String, u32>, u32) = acc;
        if let Some(value) = map.get(&x.unwrap()) {
            sum += value;
        }
        (map, sum)
    };

    let (first, second) = &reader
        .lines()
        .fold((sum1_rules1, sum2_rules2), |(x1, x2), line_raw| {
            let line: String = line_raw.unwrap();
            return (
                sum_up(x1, Ok::<String, String>(line.clone())),
                sum_up(x2, Ok(line)),
            );
        });
    let (_, sum1) = first;
    let (_, sum2) = second;
    println!("1st:{}", sum1);
    println!("2nd:{}", sum2);
}
