use aoc_lib::read_arg_file;
use std::io::{self, prelude::*, BufReader};

fn get_data() -> (Vec<Vec<char>>, Vec<(u32, usize, usize)>) {
    let reader = read_arg_file().unwrap();
    let mut first_part_vec: Vec<String> = Vec::new();
    let mut second_part_vec: Vec<String> = Vec::new();
    let mut firstpart_end: bool = false;
    for line in reader.lines() {
        if let Ok(line) = line {
            if line == "" {
                firstpart_end = true;
            } else if !firstpart_end {
                first_part_vec.push(line.clone());
            } else {
                second_part_vec.push(line.clone());
            }
        }
    }

    let bucket_count: usize = (first_part_vec.pop().unwrap().len() + 1) / 4;
    let mut ship_stacks: Vec<Vec<char>> = vec![Vec::new(); bucket_count];
    let mut movements: Vec<(u32, usize, usize)> = Vec::new();
    while let Some(line) = first_part_vec.pop() {
        for index in 0..bucket_count {
            if let Some(item) = line.chars().nth((4 * index + 1) as usize) {
                if item != ' ' {
                    ship_stacks[index].push(item);
                } else {
                }
            }
        }
    }
    for line in second_part_vec {
        let result = line
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "")
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        movements.push((result[0], result[1] as usize, result[2] as usize));
    }
    return (ship_stacks, movements);
}

fn process_movements_on_stack(stack: &mut Vec<Vec<char>>, movements: Vec<(u32, usize, usize)>) {
    for movement in movements {
        let (amount, source, destination) = movement;
        for _ in 0..amount {
            if let Some(cargo) = stack[source - 1].pop() {
                stack[destination - 1].push(cargo);
            }
        }
    }
}

fn print_upper_crates(stack: Vec<Vec<char>>) {
    for cargo_stack in stack {
        print!("{}", cargo_stack.last().unwrap());
    }
}

fn main() {
    let (mut ship_stack, movements) = get_data();
    process_movements_on_stack(&mut ship_stack, movements);
    print_upper_crates(ship_stack);
}
