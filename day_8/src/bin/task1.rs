use aoc_lib::read_arg_file;
use std::io::{self, prelude::*, BufReader};

fn check_visibility(tree_matrix: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let size_y = tree_matrix.len() as usize;
    let size_x = tree_matrix[0].len() as usize;

    let mut tree_matrix_vis: Vec<Vec<bool>> = vec![vec![false; size_x]; size_y];

    //check east west
    for y_index in 0..(size_y) {
        let mut highest = 0;
        for x_index in 0..(size_x - 1) {
            if tree_matrix[y_index][x_index] > highest {
                highest = tree_matrix[y_index][x_index];
                tree_matrix_vis[y_index][x_index] = true;
            }
        }
        tree_matrix_vis[y_index][0] = true;
        tree_matrix_vis[y_index][size_x - 1] = true;
    }

    //check west_east
    for y_index in 0..(size_y) {
        let mut highest = 0;
        for x_index in (0..(size_x)).rev() {
            if tree_matrix[y_index][x_index] > highest {
                highest = tree_matrix[y_index][x_index];
                tree_matrix_vis[y_index][x_index] = true;
            }
        }
    }

    //check north south
    for x_index in 0..(size_x - 1) {
        let mut highest = 0;
        for y_index in 0..(size_y) {
            if tree_matrix[y_index][x_index] > highest {
                highest = tree_matrix[y_index][x_index];
                tree_matrix_vis[y_index][x_index] = true;
            }
        }
        tree_matrix_vis[0][x_index] = true;
        tree_matrix_vis[size_y - 1][x_index] = true;
    }

    //check south north
    for x_index in (0..(size_x)).rev() {
        let mut highest = 0;
        for y_index in (0..(size_y)).rev() {
            if tree_matrix[y_index][x_index] > highest {
                highest = tree_matrix[y_index][x_index];
                tree_matrix_vis[y_index][x_index] = true;
            }
        }
    }

    tree_matrix_vis
}

fn tree_surrounding(
    (x_org, y_org): (usize, usize),
    tree_matrix: &Vec<Vec<u8>>,
) -> (u32, u32, u32, u32, u32) {
    let size_y = tree_matrix.len() as usize;
    let size_x = tree_matrix[0].len() as usize;

    let mut product: u32 = 1;
    // go east
    let tree_high = tree_matrix[y_org][x_org];
    let mut value1 = 0;
    let mut x = x_org;
    loop {
        if (x + 1) <= size_x - 1 {
            value1 += 1;
            if tree_matrix[y_org][x + 1] < tree_high {
                x += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    product *= value1;
    let mut value2 = 0;

    //go west
    let mut x = x_org;
    loop {
        if (x as i32 - 1) >= 0 {
            value2 += 1;
            if tree_matrix[y_org][x - 1] < tree_high {
                x -= 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    product *= value2;
    let mut value3 = 0;

    //go south
    let mut y = y_org;
    loop {
        if (y + 1) <= size_y - 1 {
            value3 += 1;
            if tree_matrix[y + 1][x_org] < tree_high {
                y += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    product *= value3;
    let mut value4 = 0;

    //go noth
    let mut y = y_org;
    loop {
        if (y as i32 - 1) >= 0 {
            value4 += 1;
            if tree_matrix[y - 1][x_org] < tree_high {
                y -= 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    product *= value4;
    (product, value1, value2, value3, value4)
}

fn main() {
    let reader = read_arg_file().unwrap();
    let mut tree_matrix: Vec<Vec<u8>> = Vec::new();

    let forest_cutter = |mut tree_matrix: Vec<Vec<u8>>, x: Result<String, _>| {
        if let Ok(tree_line) = x {
            tree_matrix.push(tree_line.chars().map(|c| c as u8 - 48).collect::<Vec<u8>>());
        }
        tree_matrix
    };
    tree_matrix = reader.lines().fold(tree_matrix, forest_cutter);
    let tree_matrix_vis = check_visibility(&tree_matrix);

    let mut sum: u32 = 0;
    let mut tree_list: Vec<(usize, usize)> = Vec::new();
    for (y_index, tree_line) in tree_matrix_vis.iter().enumerate() {
        let line_sum = 0;
        sum += tree_line.iter().enumerate().fold(line_sum, |mut acc, x| {
            let (x_index, x) = x;
            if *x == true {
                acc += 1
            } else {
                tree_list.push((x_index, y_index));
            }

            acc
        });
    }

    println!("{}", sum);

    let mut tree_value: u32 = 0;
    for x_index in 0..(tree_matrix[0].len()) {
        for y_index in 0..(tree_matrix.len()) {
            let (product, value1, value2, value3, value4) =
                tree_surrounding((x_index, y_index), &tree_matrix);
            tree_value = if product > tree_value {
                product
            } else {
                tree_value
            };
        }
    }
    println!("{:?}", tree_value);
}
