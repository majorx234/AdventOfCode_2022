use aoc_lib::read_arg_file;
use std::collections::HashMap;
use std::env::args;
use std::io::{self, prelude::*, BufReader};

fn draw_screen(signal: &Vec<i32>) {
    for y in 0..6 {
        for x in 0..40 {
            let sprite_x = signal[(y * 40 + x) as usize];
            if sprite_x - 1 == x || sprite_x == x || sprite_x + 1 == x {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let reader = read_arg_file().unwrap();
    let signal_strength: Vec<i32> =
        reader
            .lines()
            .fold(vec![1], |mut acc, x: Result<String, _>| {
                if let Ok(line) = x {
                    let cmd_value = line.split_whitespace().collect::<Vec<&str>>();
                    let cmd = cmd_value[0].chars().next().unwrap();
                    let last_value: i32 = *acc.last().unwrap();
                    if cmd == 'a' {
                        let value: i32 = cmd_value[1].parse::<i32>().unwrap();
                        acc.push(last_value);
                        acc.push(last_value + value);
                    } else if cmd == 'n' {
                        acc.push(last_value);
                    }
                }
                acc
            });
    // 20th, 60th, 100th, 140th, 180th, 220th
    let signal_strength_result = 20 * signal_strength[19]
        + 60 * signal_strength[59]
        + 100 * signal_strength[99]
        + 140 * signal_strength[139]
        + 180 * signal_strength[179]
        + 220 * signal_strength[219];
    println!("{}", signal_strength_result);
    draw_screen(&signal_strength);
}
