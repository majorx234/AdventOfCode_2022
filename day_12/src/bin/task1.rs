use aoc_lib::read_arg_file;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn get_path(
    costs: &HashMap<(usize, usize), (i32, (usize, usize))>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path: Vec<(usize, usize)> = vec![goal.clone()];
    let mut node = goal;
    while node != start {
        let pre_node = costs.get(&node).unwrap().1;
        path.push(pre_node.clone());
        node = pre_node;
    }
    return path;
}

fn next_minimal_node(
    costs: &HashMap<(usize, usize), (i32, (usize, usize))>,
    visited_nodes: &HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut canditate = (usize::MAX, usize::MAX);
    let mut current_min = -1;
    for key in costs.keys() {
        let value = costs.get(key).unwrap().0;
        if value != -1 {
            if ((current_min == -1) || (current_min > value)) && !visited_nodes.contains(key) {
                current_min = value;
                canditate = key.clone();
            }
        }
    }

    return canditate;
}

fn find_path(
    graph: HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> (
    (usize, usize),
    i32,
    HashMap<(usize, usize), (i32, (usize, usize))>,
) {
    // initialize
    let mut unvisted_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut costs: HashMap<(usize, usize), (i32, (usize, usize))> = HashMap::new();
    let mut node = start;
    let mut not_cancel: bool = true;

    for key_node in graph.keys() {
        unvisted_nodes.insert(key_node.clone());
        costs.insert(key_node.clone(), (-1, (usize::MAX, usize::MAX)));
    }

    *costs.entry(node.clone()).or_insert((0, node.clone())) = (0, node.clone());

    while not_cancel {
        let cost = costs.get(&node.clone()).unwrap().0;
        let neighbor_values = graph.get(&node.clone()).unwrap();
        let neighbors: HashSet<(usize, usize)> = HashSet::new();
        for neighbor in neighbor_values {
            let distance = 1;
            let start_to_neighbor_cost = distance + cost.clone();
            let new_node_costs = (start_to_neighbor_cost, node.clone());
            if -1 == costs.get(&neighbor.clone()).unwrap().0 {
                *costs.entry(*neighbor).or_insert(new_node_costs) = new_node_costs;
            } else if start_to_neighbor_cost < costs.get(&neighbor.clone()).unwrap().0 {
                *costs.entry(*neighbor).or_insert(new_node_costs) = new_node_costs;
            }
        }
        visited_nodes.insert(node);
        node = next_minimal_node(&costs, &visited_nodes);
        if node == (usize::MAX, usize::MAX) {
            not_cancel = false;
        }
        println!("{:?}", node);
    }
    return (
        costs.get(&goal.clone()).unwrap().1,
        costs.get(&goal.clone()).unwrap().0,
        costs,
    );
}

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

    for x_index in 0..x_len {
        for y_index in 0..(y_len - 1) {
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
    let input_graph = build_graph(input_map);
    println!("{:?}", input_graph);
}
