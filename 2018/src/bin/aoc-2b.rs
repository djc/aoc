use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut ids: Vec<_> = BufReader::new(File::open("input/2.txt")?)
        .lines()
        .map(|ln| ln.unwrap())
        .collect();
    ids.sort_unstable();

    // See also: Itertools::cartesian_product()
    let mut found = None;
    'outer: for id1 in ids.iter() {
        for id2 in ids.iter() {
            let diffs = diff(id1, id2);
            if diffs > 0 && diffs < 2 {
                found = Some((id1, id2));
                break 'outer;
            }
        }
    }

    let (s1, s2) = found.unwrap();
    let mut result = String::new();
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            result.push(c1);
        }
    }

    println!("result: {}", result);
    Ok(())
}

fn diff(s1: &str, s2: &str) -> usize {
    let mut diffs = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diffs += 1;
        }
        if diffs > 1 {
            break;
        }
    }
    diffs
}
