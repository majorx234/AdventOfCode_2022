use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    multi::{count, separated_list1},
    IResult,
};
use std::env::args;
use std::{fs::read_to_string, path::Path};

#[derive(Debug, PartialEq)]
enum Signal {
    Value(i32),
    List(Vec<Signal>),
    None,
}

fn compare_signals(left: &Signal, right: &Signal) -> Option<bool> {
    let bool_value: bool = false;
    match left {
        Signal::Value(value_left) => match right {
            Signal::Value(value_right) => {
                if value_left < value_right {
                    return Some(true);
                } else if value_left > value_right {
                    return Some(false);
                } else {
                    return None;
                }
            }
            Signal::List(vec_values_right) => {
                return compare_signals(
                    &Signal::List(vec![Signal::Value((value_left).clone())]),
                    right,
                );
            }
            Signal::None => {
                println!("right side went out of elements");
                return Some(false);
            }
        },
        Signal::List(vec_values_left) => match right {
            Signal::Value(value_right) => {
                return compare_signals(
                    left,
                    &Signal::List(vec![Signal::Value(value_right.clone())]),
                );
            }
            Signal::List(vec_values_right) => {
                let mut comparer: Option<bool> = None;
                let mut iter_left = vec_values_left.iter();
                let mut iter_right = vec_values_right.iter();
                loop {
                    if let Some(left_item) = iter_left.next() {
                        if let Some(right_item) = iter_right.next() {
                            comparer = compare_signals(left_item, right_item);
                            match comparer {
                                Some(_) => {
                                    return comparer;
                                }
                                None => {}
                            }
                        } else {
                            // right side run out of date
                            return Some(false);
                        }
                    } else {
                        if let Some(right_item) = iter_right.next() {
                            // left side run out of items
                            return Some(true);
                        } else {
                            return None;
                        }
                    }
                }
            }
            Signal::None => {
                println!("right side went out of elements");
                return Some(true);
            }
        },
        Signal::None => {
            println!("left side went out of elements");
            return Some(true);
        }
    }
    println!("case is not covered yet");
    None
}

fn parse_input(input: &str) -> Vec<(Signal, Signal)> {
    let mut signals: Vec<(Signal, Signal)> = Vec::new();
    let mut signal_temp: (Signal, Signal) = (Signal::None, Signal::None);
    let mut helper_stack: Vec<Signal> = Vec::new();
    let mut new = Signal::None;
    //let mut first: bool = true;
    let mut bool_new_item = false;
    let mut bool_new_numeric = false;
    let mut numeric: String = "".to_string();
    let mut numeric_str = "".to_string();
    let mut end: bool = false;
    let mut state: usize = 0;

    for char_item in input.chars() {
        if char_item == '[' {
            if new != Signal::None {
                helper_stack.push(new);
            }
            new = Signal::List(Vec::new());
        } else if char_item == ']' {
            if bool_new_numeric {
                let value = Signal::Value(numeric_str.parse::<i32>().unwrap());
                numeric_str = "".to_string();
                match new {
                    Signal::Value(_) => new = value,
                    Signal::List(ref mut new_vec) => new_vec.push(value),
                    Signal::None => new = value,
                };
                bool_new_numeric = false;
            }
            let old: Signal = new;
            if let Some(stack_elem) = helper_stack.pop() {
                new = stack_elem;
                match new {
                    Signal::Value(new_value) => {
                        let mut new_new = vec![Signal::Value(new_value)];
                        new_new.push(old);
                        new = Signal::List(new_new);
                    }
                    Signal::List(ref mut new_vec) => new_vec.push(old),
                    Signal::None => (),
                }
            } else {
                new = old;
            }
        } else if char_item == ',' {
            if bool_new_numeric {
                let value = Signal::Value(numeric_str.parse::<i32>().unwrap());
                numeric_str = "".to_string();
                match new {
                    Signal::Value(_) => new = value,
                    Signal::List(ref mut new_vec) => new_vec.push(value),
                    Signal::None => new = value,
                };
                bool_new_numeric = false;
            }
        } else if char_item.is_numeric() {
            if bool_new_numeric {
                numeric_str.push(char_item);
            } else {
                //numeric_str = String::from(numeric);
                numeric_str.push(char_item);
                //numeric = numeric_str;
                bool_new_numeric = true;
            }
        } else if char_item == '\n' {
            if state == 0 {
                signal_temp.0 = new;
                new = Signal::None;
            } else if state == 1 {
                signal_temp.1 = new;
                new = Signal::None;
            } else {
                signals.push(signal_temp);
                signal_temp = (Signal::None, Signal::None);
            }
            state = (state + 1) % 3;
        } else {
            println!("error char {} not known", char_item);
        }
    }

    if state == 2 {
        signals.push(signal_temp);
    }
    signals
}

fn main() {
    let mut argit = args();
    let file_name = argit.nth(1).clone();
    let mut input: String = "".to_string();

    if let Some(file_name) = file_name {
        Path::new(&file_name);
        input = read_to_string(file_name).unwrap();
    } else {
        panic!("No filename argument given");
    };
    let signals = parse_input(&input);
}
