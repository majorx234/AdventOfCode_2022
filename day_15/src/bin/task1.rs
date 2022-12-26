use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    multi::{count, separated_list1},
    IResult,
};
use std::collections::HashMap;
use std::env::args;
use std::{fs::read_to_string, path::Path};

fn print_row_vec(row_map: Vec<bool>) {
    for item in row_map {
        if item {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

fn print_row_map(row_map: HashMap<i32, bool>) {
    /*
    for item in row_map {
        println!("{}#", item.0);
    }*/
    println!("task1:{}", row_map.keys().len() - 1);
}

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

fn manhatten_dist(s_x: i32, s_y: i32, b_x: i32, b_y: i32) -> i32 {
    let x_diff = if s_x < b_x { b_x - s_x } else { s_x - b_x };
    let y_diff = if s_y < b_y { b_y - s_y } else { s_y - b_y };
    x_diff + y_diff
}

fn get_x_min_max_beacon(sensor_beacons: &Vec<(i32, i32, i32, i32)>) -> (i32, i32) {
    let mut min = sensor_beacons[0].3;
    let mut max = sensor_beacons[0].3;
    for (s_x, s_y, b_x, b_y) in sensor_beacons {
        if *b_x < min {
            min = *b_x;
        }
        if *b_x > max {
            max = *b_x
        }
    }
    (min, max)
}

fn no_bacon_posision(row: i32, sensor_beacons: Vec<(i32, i32, i32, i32)>) -> i32 {
    let (min_x, max_x) = get_x_min_max_beacon(&sensor_beacons);
    let mut dist_vec: Vec<i32> = Vec::new();
    let mut row_map: HashMap<i32, bool> = HashMap::new();
    for (s_x, s_y, b_x, b_y) in sensor_beacons {
        let manhatten_dist = manhatten_dist(s_x, s_y, b_x, b_y);
        let row_dist = if row < s_y { s_y - row } else { row - s_y };
        if row_dist < manhatten_dist {
            let index_x = (s_x - min_x);
            let occupied_elements = manhatten_dist - row_dist;

            row_map.entry(index_x).or_insert(true);

            for index in 1..(occupied_elements + 1) {
                row_map.entry(index_x - index).or_insert(true);

                row_map.entry(index_x + index).or_insert(true);
            }
            dist_vec.push(manhatten_dist);
        } else {
            dist_vec.push(-1);
        }
    }
    print_row_map(row_map);
    0
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32, i32, i32)>, ()> {
    let (input, sensor_beacons) = separated_list1(newline, parse_sensorbeacon)(input)?;
    Ok((input, sensor_beacons))
}

fn main() {
    let mut argit = args();
    let file_name = argit.nth(1).clone();
    let row_no = argit.next().clone();
    let mut input: String = "".to_string();
    if let Some(file_name) = file_name {
        Path::new(&file_name);
        input = read_to_string(file_name).unwrap();
    } else {
        panic!("No filename argument given");
    };

    let row_no = if let Some(row_no) = row_no {
        if let Ok(row_no) = str::parse::<i32>(&row_no) {
            row_no
        } else {
            panic!("row_no isn't given as i32 value");
        }
    } else {
        panic!("No row_no argument given");
    };

    let (_, mut sensor_beacons) = parse_input(&input).unwrap();
    println!("{:?}", sensor_beacons);
    no_bacon_posision(row_no, sensor_beacons);
}
