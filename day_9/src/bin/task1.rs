use aoc_lib::read_arg_file;
use std::collections::HashMap;
use std::env::args;
use std::io::{self, prelude::*, BufReader};

fn calc_tail_move(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    let diff_x = head_x - tail_x;
    let diff_y = head_y - tail_y;
    println!("diff: {}, {}", diff_x, diff_y);
    let (delta_x, delta_y) = if (i32::abs(diff_y) == 2) && (i32::abs(diff_x) == 2) {
        println!("error");
        (diff_x / 2, diff_y / 2)
    } else if i32::abs(diff_x) > 1 {
        (diff_x / 2, diff_y)
    } else if i32::abs(diff_y) > 1 {
        (diff_x, diff_y / 2)
    } else {
        (0, 0)
    };

    (tail_x + delta_x, tail_y + delta_y)
}

fn simulate_movement(
    nodes: &mut Vec<(i32, i32)>,
    move_step: (char, u32),
    footprints: &mut HashMap<(i32, i32), bool>,
) {
    let (moved, steps) = move_step;

    let rope_length = nodes.len();
    for _ in 0..steps {
        if moved == 'R' {
            nodes[0].0 += 1;
        } else if moved == 'U' {
            nodes[0].1 += 1;
        } else if moved == 'L' {
            nodes[0].0 -= 1;
        } else if moved == 'D' {
            nodes[0].1 -= 1;
        } else {
            // no normal direction
        }

        for tail_steps in 0..(rope_length - 1) {
            let (mut head_x, mut head_y) = nodes[tail_steps];
            let (mut tail_x, mut tail_y) = nodes[tail_steps + 1];

            (tail_x, tail_y) = calc_tail_move(head_x, head_y, tail_x, tail_y);
            if tail_steps == rope_length - 2 {
                footprints.insert((tail_x, tail_y), true);
            }
            nodes[tail_steps + 1] = (tail_x, tail_y)
        }
    }
}

fn main() {
    let mut argit = args();
    let rope_length = argit.nth(2).clone();
    let rope_length = if let Some(rope_length) = rope_length {
        if let Ok(rope_length) = str::parse::<u32>(&rope_length) {
            rope_length
        } else {
            panic!("rope_length isn't given as u32 value");
        }
    } else {
        panic!("No rope_length argument given");
    };
    let reader = read_arg_file().unwrap();
    let head_move: Vec<(char, u32)> =
        reader
            .lines()
            .fold(Vec::new(), |mut acc, x: Result<String, _>| {
                if let Ok(line) = x {
                    let split = line.split_whitespace().collect::<Vec<&str>>();
                    let movement = split[0].chars().next().unwrap();
                    let steps = split[1].parse::<u32>().unwrap();
                    acc.push((movement, steps));
                }
                acc
            });

    let mut footprints: HashMap<(i32, i32), bool> = HashMap::new();
    let mut nodes: Vec<(i32, i32)> = vec![(0, 0); rope_length as usize];

    for move_step in head_move {
        simulate_movement(&mut nodes, move_step, &mut footprints);
    }
    println!("{}", footprints.keys().len());
}
