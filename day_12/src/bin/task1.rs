use std::collections::HashMap;
use std::io::BufRead;

use aoc_lib::read_arg_file;
fn val(ch: char) -> u32 {
    let mut ch_val = ch as u32;
    if ch_val == 'E' as u32 {
        ch_val = 27;
    }
    if ch_val == 'S' as u32 {
        ch_val = 0;
    }
    ch_val
}

fn build_graph(input_map: Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let y_len: usize = input_map.len();
    let x_len: usize = input_map[0].len();
    for y_index in 0..y_len {
        for x_index in 0..(x_len - 1) {
            let value_l = val(input_map[y_index][x_index]);
            let value_r = val(input_map[y_index][x_index + 1]);
            if value_l < value_r {
                if !graph.contains_key(&(x_index + 1, y_index)) {
                    graph.insert((x_index + 1, y_index), vec![(x_index, y_index)]);
                } else {
                    graph
                        .get_mut(&(x_index + 1, y_index))
                        .unwrap()
                        .push((x_index, y_index));
                }

                if value_r - value_l <= 1 {
                    if !graph.contains_key(&(x_index, y_index)) {
                        graph.insert((x_index, y_index), vec![(x_index + 1, y_index)]);
                    } else {
                        graph
                            .get_mut(&(x_index, y_index))
                            .unwrap()
                            .push((x_index + 1, y_index));
                    }
                }
            } else {
                if !graph.contains_key(&(x_index, y_index)) {
                    graph.insert((x_index, y_index), vec![(x_index + 1, y_index)]);
                } else {
                    graph
                        .get_mut(&(x_index, y_index))
                        .unwrap()
                        .push((x_index + 1, y_index));
                }

                if value_l - value_r <= 1 {
                    if !graph.contains_key(&(x_index + 1, y_index)) {
                        graph.insert((x_index, y_index), vec![(x_index, y_index)]);
                    } else {
                        graph
                            .get_mut(&(x_index + 1, y_index))
                            .unwrap()
                            .push((x_index, y_index));
                    }
                }
            }
        }
    }

    for x_index in 0..y_len {
        for y_index in 0..(x_len - 1) {
            let value_o = val(input_map[y_index][x_index]);
            let value_u = val(input_map[y_index + 1][x_index]);
            if value_u < value_o {
                if !graph.contains_key(&(x_index, y_index)) {
                    graph.insert((x_index, y_index), vec![(x_index, y_index + 1)]);
                } else {
                    graph
                        .get_mut(&(x_index, y_index))
                        .unwrap()
                        .push((x_index, y_index + 1));
                }

                if value_o - value_u <= 1 {
                    if !graph.contains_key(&(x_index, y_index + 1)) {
                        graph.insert((x_index, y_index), vec![(x_index, y_index)]);
                    } else {
                        graph
                            .get_mut(&(x_index, y_index + 1))
                            .unwrap()
                            .push((x_index, y_index));
                    }
                }
            } else {
                if !graph.contains_key(&(x_index, y_index + 1)) {
                    graph.insert((x_index, y_index), vec![(x_index, y_index)]);
                } else {
                    graph
                        .get_mut(&(x_index, y_index + 1))
                        .unwrap()
                        .push((x_index, y_index));
                }

                if value_u - value_o <= 1 {
                    if !graph.contains_key(&(x_index, y_index)) {
                        graph.insert((x_index, y_index), vec![(x_index, y_index + 1)]);
                    } else {
                        graph
                            .get_mut(&(x_index, y_index))
                            .unwrap()
                            .push((x_index, y_index + 1));
                    }
                }
            }
        }
    }
    graph
}

fn main() {
    let reader = read_arg_file().unwrap();
    let input_map: Vec<Vec<char>> =
        reader
            .lines()
            .fold(Vec::new(), |mut acc, x: Result<String, _>| {
                if let Ok(x) = x {
                    acc.push(x.chars().collect::<Vec<char>>());
                }
                acc
            });
    println!("{:?}", input_map);
}
