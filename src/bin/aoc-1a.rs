use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let val = BufReader::new(File::open("input/1.txt")?)
        .lines()
        .fold(0isize, |acc, res| {
            acc + res.unwrap().parse::<isize>().unwrap()
        });
    println!("Value {}", val);
    Ok(())
}
