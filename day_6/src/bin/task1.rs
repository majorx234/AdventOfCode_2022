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

    if let Some(Ok(line)) = read_arg_file().unwrap().lines().nth(0) {
        for (index, window) in line
            .chars()
            .collect::<Vec<char>>()
            .windows(window_size)
            .enumerate()
        {
            let has_dublicate_fct = |mut hashmap: HashMap<char, bool>, x: &char| {
                if let None = hashmap.insert(*x, true) {
                    return Some(hashmap);
                }
                None
            };
            let has_no_dublicate = window.iter().try_fold(HashMap::new(), has_dublicate_fct);
            if has_no_dublicate != None {
                print!("index: {}\n", index + window_size);
                break;
            }
        }
    }
}
