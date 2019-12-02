use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let counts =
        BufReader::new(File::open("input/2.txt")?)
            .lines()
            .fold((0, 0), |mut counts, res| {
                let mut freq = HashMap::new();
                let ln = res.unwrap();
                for c in ln.chars() {
                    let count = freq.entry(c).or_insert(0);
                    *count += 1;
                }

                let mut found = (false, false);
                for val in freq.values() {
                    if *val == 2 {
                        found.0 = true;
                    } else if *val == 3 {
                        found.1 = true;
                    }
                }

                if found.0 {
                    counts.0 += 1;
                }
                if found.1 {
                    counts.1 += 1;
                }

                println!("line {:?} -> {:?} -> {:?}", ln, freq, counts);
                counts
            });
    println!("Counts {:?} -> {}", counts, counts.0 * counts.1);
    Ok(())
}
