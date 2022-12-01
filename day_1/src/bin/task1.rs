use aoc_lib::read_arg_file;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();
    let max123_elfsum_id: (u32, u32, u32, u32, u32) = (0, 0, 0, 0, 0);
    let fold_fct_task = |acc, x: Result<String, _>| {
        if let Ok(x) = x {
            let (max1, max2, max3, elfsum, id) = acc;
            if x == "" {
                if max1 < elfsum {
                    return (elfsum, max1, max2, 0, id + 1);
                } else if max2 < elfsum {
                    return (max1, elfsum, max2, 0, id + 1);
                } else if max3 < elfsum {
                    return (max1, max2, elfsum, 0, id + 1);
                } else {
                    return (max1, max2, max3, 0, id + 1);
                }
            } else if let x = x.parse::<u32>().unwrap() {
                return (max1, max2, max3, elfsum + x, id);
            } else {
                return (max1, max2, max3, elfsum, id);
            }
        }
        acc
    };
    let fourtytwo = reader.lines().fold(max123_elfsum_id, fold_fct_task);
    let (max1, max2, max3, _, _) = fourtytwo;
    println!("1st:{}", max1);
    println!("2nd:{}", max1 + max2 + max3);
}
