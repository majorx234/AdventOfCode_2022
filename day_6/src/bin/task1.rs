use aoc_lib::read_arg_file;
use std::collections::HashMap;
use std::env::args;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let mut argit = args();
    let window_size = argit.nth(2).clone();
    let window_size = if let Some(window_size) = window_size {
        if let Ok(window_size) = str::parse::<usize>(&window_size) {
            window_size
        } else {
            panic!("window_size isn't given as u32 value");
        }
    } else {
        panic!("No window_size argument given");
    };

    let reader = read_arg_file().unwrap();
    if let Some(Ok(line)) = reader.lines().nth(0) {
        for (index, window) in line
            .chars()
            .collect::<Vec<char>>()
            .windows(window_size)
            .enumerate()
        {
            let has_dublicate_fct = |acc, x: &char| {
                let (has_dublicate, mut hashmap): (bool, HashMap<char, bool>) = acc;
                if let None = hashmap.insert(*x, true) {
                    (has_dublicate, hashmap)
                } else {
                    (true, hashmap)
                }
            };
            let (has_dublicate, _) = window
                .iter()
                .fold((false, HashMap::new()), has_dublicate_fct);
            if !has_dublicate {
                print!("index: {}\n", index + window_size);
                break;
            }
        }
    }
}
