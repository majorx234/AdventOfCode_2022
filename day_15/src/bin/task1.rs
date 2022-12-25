use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    multi::{count, separated_list1},
    IResult,
};
use std::env::args;
use std::{fs::read_to_string, path::Path};

fn parse_sensorbeacon(input: &str) -> IResult<&str, (i32, i32, i32, i32), ()> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x_coordinate) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y_coordinate) = complete::i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x_coordinate) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y_coordinate) = complete::i32(input)?;
    Ok((
        input,
        (
            sensor_x_coordinate,
            sensor_y_coordinate,
            beacon_x_coordinate,
            beacon_y_coordinate,
        ),
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32, i32, i32)>, ()> {
    let (input, sensor_beacons) = separated_list1(newline, parse_sensorbeacon)(input)?;
    Ok((input, sensor_beacons))
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
    let (_, mut sensor_beacons) = parse_input(&input).unwrap();
    println!("{:?}", sensor_beacons);
}
