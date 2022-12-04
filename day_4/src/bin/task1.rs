use aoc_lib::read_arg_file;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();
    let sum: u32 = 0;
    let fold_fct_task = |acc, x: Result<String, _>| {
        if let Ok(x) = x {
            let split: Vec<&str> = x.split(",").collect();
            let elf1: Vec<&str> = split[0].split("-").collect();
            let elf2: Vec<&str> = split[1].split("-").collect();
            let elf1_low_bound = elf1[0].to_string().parse::<u32>().unwrap();
            let elf1_high_bound = elf1[1].to_string().parse::<u32>().unwrap();
            let elf2_low_bound = elf2[0].to_string().parse::<u32>().unwrap();
            let elf2_high_bound = elf2[1].to_string().parse::<u32>().unwrap();

            if (elf1_low_bound <= elf2_low_bound && elf1_high_bound >= elf2_high_bound)
                || (elf1_low_bound >= elf2_low_bound && elf1_high_bound <= elf2_high_bound)
            {
                return acc + 1;
            }
        }
        acc
    };

    let fourtytwo = reader.lines().fold(sum, fold_fct_task);
    println!("First: {}", fourtytwo);
}
