use aoc_lib::read_arg_file;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();
    let max_elfsum_id: (u32, u32, u32) = (0, 0, 0);
    let fourtytwo = reader.lines().fold(max_elfsum_id, |acc, x| {
        if let Ok(x) = x {
            let (max, elfsum, id) = acc;
            if x == "" {
                if max < elfsum {
                    return (elfsum, 0, id + 1);
                } else {
                    return (max, 0, id + 1);
                }
            } else if let x = x.parse::<u32>().unwrap() {
                return (max, elfsum + x, id);
            } else {
                return (max, elfsum, id);
            }
        }
        acc
    });
    let (max_elf, _, _) = fourtytwo;
    println!("{}", max_elf);
}
