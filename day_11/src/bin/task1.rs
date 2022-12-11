use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    multi::{count, separated_list1},
    IResult,
};
use std::env::args;
use std::{fs::read_to_string, path::Path};

#[derive(Debug)]
struct Formular {
    first: FormularElem,
    operation: Operation,
    second: FormularElem,
}

#[derive(Debug, PartialEq)]
enum FormularElem {
    Old,
    Value(u64),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    items: Vec<u64>,
    formular: Formular,
    divisor: u64,
    monkey_true: u64,
    monkey_false: u64,
}

fn parse_formular(input: &str) -> IResult<&str, Formular, ()> {
    let (input, _) = tag("old ")(input)?;
    let operation_char = input.chars().nth(0).unwrap();
    let operation = if operation_char == '+' {
        Operation::Add
    } else {
        Operation::Mul
    };
    let input = &input[2..];
    if input.chars().nth(0).unwrap() == 'o' {
        let (input, _) = tag("old")(input)?;
        let first = FormularElem::Old;
        let second = FormularElem::Old;
        let formular = Formular {
            first: first,
            operation: operation,
            second: second,
        };
        Ok((input, formular))
    } else {
        let (input, second_var) = complete::u64(input)?;
        let first = FormularElem::Old;
        let second = FormularElem::Value(second_var);
        let formular = Formular {
            first: first,
            operation: operation,
            second: second,
        };
        Ok((input, formular))
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey, ()> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, id) = complete::u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, item_list) = separated_list1(tag(", "), complete::u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("  Operation: new = ")(input)?;
    let (input, formular) = parse_formular(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("  Test: divisible by ")(input)?;
    let (input, divisor) = complete::u64(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    let (input, monkey_true) = complete::u64(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("    If false: throw to monkey ")(input)?;
    let (input, monkey_false) = complete::u64(input)?;
    let monkey = Monkey {
        id: id,
        items: item_list,
        formular: formular,
        divisor: divisor,
        monkey_true: monkey_true,
        monkey_false: monkey_false,
    };
    Ok((input, monkey))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>, ()> {
    let (input, monkeys) = separated_list1(count(newline, 2), parse_monkey)(input)?;
    Ok((input, monkeys))
}

fn keep_away(
    monkeys: &mut Vec<Monkey>,
    monkey_inspections: &mut Vec<u64>,
    modulo: u64,
    worry_divider: u64,
) {
    for index in 0..monkeys.len() {
        let mut monkey = &mut monkeys[index];
        let mut thrown_items: Vec<(u64, usize)> = Vec::new();
        for item in monkey.items.iter_mut() {
            let item = if monkey.formular.operation == Operation::Add {
                match monkey.formular.second {
                    FormularElem::Old => (*item + *item) / worry_divider,
                    FormularElem::Value(value) => (*item + value) / worry_divider,
                }
            } else {
                match monkey.formular.second {
                    FormularElem::Old => (*item * *item) / worry_divider,
                    FormularElem::Value(value) => (*item * value) / worry_divider,
                }
            };
            monkey_inspections[index] += 1;

            if item % monkey.divisor == 0 {
                thrown_items.push((item, monkey.monkey_true as usize));
            //                monkeys[monkey.monkey_true as usize].items.push(item);
            } else {
                thrown_items.push((item, monkey.monkey_false as usize));
            }
        }
        monkey.items = Vec::new();
        for (item, index) in thrown_items {
            monkeys[index].items.push((item % modulo));
        }
    }
}

fn main() {
    let mut argit = args();
    let file_name = argit.nth(1).clone();
    let rounds = argit.next().clone();
    let worry_divider = argit.next().clone();

    let mut input: String = "".to_string();

    if let Some(file_name) = file_name {
        Path::new(&file_name);
        input = read_to_string(file_name).unwrap();
    } else {
        panic!("No filename argument given");
    };

    let rounds = if let Some(rounds) = rounds {
        if let Ok(rounds) = str::parse::<u64>(&rounds) {
            rounds
        } else {
            panic!("rounds isn't given as u64 value");
        }
    } else {
        panic!("No filename argument given");
    };

    let worry_divider = if let Some(worry_divider) = worry_divider {
        if let Ok(worry_divider) = str::parse::<u64>(&worry_divider) {
            worry_divider
        } else {
            panic!("worry_divider isn't given as u64 value");
        }
    } else {
        panic!("No filename argument given");
    };

    let (_, mut monkeys) = parse_input(&input).unwrap();
    let mounkey_count = monkeys.len();
    let modulo = monkeys.iter().fold(1, |acc, monkey| monkey.divisor * acc);
    let mut monkey_inspections: Vec<u64> = vec![0; mounkey_count];

    for round in 0..rounds {
        keep_away(&mut monkeys, &mut monkey_inspections, modulo, worry_divider);
    }
    println!("round {} {:?})", 20, monkey_inspections);
    monkey_inspections.sort();
    let first_monkey = monkey_inspections.pop().unwrap();
    let second_monkey = monkey_inspections.pop().unwrap();
    println!("{}", first_monkey * second_monkey);
}
