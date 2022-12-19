use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    multi::{count, separated_list1},
    IResult,
};
use std::env::args;
use std::{fs::read_to_string, path::Path};

#[derive(Debug)]
struct RockStruct {
    id: u32,
    points: Vec<(i32, i32)>,
}

fn parse_point(input: &str) -> IResult<&str, (i32, i32), ()> {
    let (input, x_coordinate) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y_coordinate) = complete::i32(input)?;
    Ok((input, (x_coordinate, y_coordinate)))
}
fn parse_rockstruct(input: &str) -> IResult<&str, RockStruct, ()> {
    let (input, points) = separated_list1(tag(" -> "), parse_point)(input)?;
    Ok((
        input,
        RockStruct {
            id: 0,
            points: points,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<RockStruct>, ()> {
    let (input, lines) = separated_list1(newline, parse_rockstruct)(input)?;
    Ok((input, lines))
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
    let (_, mut rockstructs) = parse_input(&input).unwrap();
    println!("{:?}", rockstructs);
}
