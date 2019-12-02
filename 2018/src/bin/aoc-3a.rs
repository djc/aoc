use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let specs: Vec<_> = BufReader::new(File::open("input/3-test.txt")?)
        .lines()
        .map(|ln| {
            let ln = ln.unwrap();
            let mut parts = ln.split(" ");
            let id = parts.next().unwrap()[1..].parse().unwrap();
            let _ = parts.next();
            let mut start = parts.next().unwrap().split(",");
            let left = start.next().unwrap().parse().unwrap();
            let top = start.next().unwrap();
            let top = top[..top.len() - 1].parse().unwrap();
            let mut area = parts.next().unwrap().split("x");
            let width = area.next().unwrap().parse().unwrap();
            let height = area.next().unwrap().parse().unwrap();
            Spec {
                id,
                left,
                top,
                width,
                height,
            }
        })
        .collect();

    let mut used = HashSet::new();
    let mut dupes = 0;
    for spec in &specs {
        println!("spec {:?}", spec);
        for row in 0..spec.height {
            for col in 0..spec.width {
                let coords = (spec.left + col, spec.top + row);
                let dupe = !used.insert(coords);
                println!("INSERT {:?} -> {:?}", coords, dupe);
                if dupe {
                    dupes += 1;
                }
            }
        }
    }
    
    println!("duplicates: {}", dupes);
    Ok(())
}

#[derive(Debug)]
struct Spec {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}
