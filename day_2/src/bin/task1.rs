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

    let sum1_rules1: (HashMap<String, u32>, u32) = (rules1, 0);
    let sum_up = |acc, x: Result<String, _>| {
        let (map, mut sum): (HashMap<String, u32>, u32) = acc;
        sum += map.get(&x.unwrap()).unwrap();
        (map, sum)
    };

    let first = reader.lines().fold(sum1_rules1, sum_up);
    let (_, sum1) = first;
    let second = "nothing yet";
    println!("1st:{}", sum1);
    println!("2nd:{}", second);
}
