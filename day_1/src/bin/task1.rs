use std::env::args;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut argit = args();
    let file_name = argit.nth(1).clone();

    let file_name = if let Some(file_name) = file_name {
        file_name
    } else {
        panic!("No filename argument given");
    };

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let max_elfsum_id: (u32, u32, u32) = (0, 0, 0);
    let fourtytwo = reader.lines().fold(max_elfsum_id, |acc, x| {
        if let Ok(x) = x {
            let (max, elfsum, id) = acc;
            if x == "" {
                if max < elfsum {
                    return (elfsum, 0, id + 1);
                } else {
                    return (max, 0, id + 1);
                }
            } else if let x = x.parse::<u32>().unwrap() {
                return (max, elfsum + x, id);
            } else {
                return (max, elfsum, id);
            }
        }
        acc
    });
    println!("{:?}", fourtytwo);
    Ok(())
}
