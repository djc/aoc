use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let val = BufReader::new(File::open("input/1.txt")?).lines().fold(0isize, |acc, res| {
        acc + res.unwrap().parse::<isize>().unwrap()
    });
    println!("Value {}", val);
    Ok(())
}
