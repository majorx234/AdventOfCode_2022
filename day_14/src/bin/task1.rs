use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    multi::{count, separated_list1},
    IResult,
};
use std::env::args;
use std::{fs::read_to_string, path::Path};

#[derive(Clone, Debug, PartialEq)]
enum Material {
    Empty,
    Rock,
    Sand,
}

#[derive(Debug)]
struct RockStruct {
    points: Vec<(i32, i32)>,
}

#[derive(Debug)]
struct Cave {
    offset_x: i32,
    offset_y: i32,
    tiles: Vec<Vec<Material>>,
}

impl Cave {
    fn print(&self) {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if self.tiles[y][x] == Material::Rock {
                    print!("#");
                } else if self.tiles[y][x] == Material::Sand {
                    print!("o");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn parse_point(input: &str) -> IResult<&str, (i32, i32), ()> {
    let (input, x_coordinate) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y_coordinate) = complete::i32(input)?;
    Ok((input, (x_coordinate, y_coordinate)))
}
fn parse_rockstruct(input: &str) -> IResult<&str, RockStruct, ()> {
    let (input, points) = separated_list1(tag(" -> "), parse_point)(input)?;
    Ok((input, RockStruct { points: points }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<RockStruct>, ()> {
    let (input, lines) = separated_list1(newline, parse_rockstruct)(input)?;
    Ok((input, lines))
}

fn interpolate(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;
    let min_x = start_x.min(end_x);
    let max_x = start_x.max(end_x);
    let min_y = start_y.min(end_y);
    let max_y = start_y.max(end_y);
    let delta_x = start_x.abs_diff(end_x) as usize;
    let delta_y = start_y.abs_diff(end_y) as usize;
    if delta_x != 0 {
        (min_x..(max_x + 1)).zip(vec![min_y; delta_x + 1]).collect()
    } else {
        vec![min_x; delta_y + 1]
            .into_iter()
            .zip(min_y..(max_y + 1))
            .collect()
    }
}

fn generate_cave_model(rockstructs: Vec<RockStruct>) -> Option<Cave> {
    // find min_x, max_x, min_y, max_y
    let (mut min_x, mut min_y) = rockstructs[0].points[0];
    let (mut max_x, mut max_y) = rockstructs[0].points[0];
    for rockstruct in &rockstructs {
        for (x, y) in &rockstruct.points {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
        }
    }

    let size_x = ((max_x - min_x) + 1) as usize;
    let size_y = ((max_y - min_y) + 1) as usize;
    let offset_x = min_x;
    let offset_y = min_y;
    let mut cave_model = Cave {
        offset_x: offset_x,
        offset_y: offset_y,
        tiles: vec![vec![Material::Empty; size_x]; size_y],
    };
    for rockstruct in rockstructs {
        for index in 0..(rockstruct.points.len() - 1) {
            let start = rockstruct.points[index];
            let end = rockstruct.points[index + 1];

            let rock_tiles = interpolate(start, end);

            for (x, y) in rock_tiles {
                cave_model.tiles[(y - offset_y) as usize][(x - offset_x) as usize] = Material::Rock;
            }
        }
    }
    cave_model.print();

    Some(cave_model)
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
    generate_cave_model(rockstructs);
}
