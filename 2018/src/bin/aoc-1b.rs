use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut observed = HashSet::new();
    let values: Vec<isize> = BufReader::new(File::open("input/1.txt")?)
        .lines()
        .map(|ln| ln.unwrap().parse().unwrap())
        .collect();
    let val = values.iter().cycle().try_fold(0isize, |acc, new| {
        let res = acc + new;
        let seen = observed.insert(res);
        if seen {
            Ok(res)
        } else {
            Err(res)
        }
    });
    println!("Value {:?}", val);
    Ok(())
}
