use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::mem;

fn main() -> std::io::Result<()> {
    let raw = read_to_string("input/6")?;
    let mut orbits = HashMap::new();
    let mut orbited = HashMap::new();

    for edge in raw.trim().split('\n') {
        if edge.len() == 0 {
            continue;
        }
        assert_eq!(edge.len(), 7);
        let pos = edge.as_bytes().iter().position(|b| *b == b')').unwrap();
        let (src, dst) = (&edge[..pos], &edge[pos + 1..]);
        orbits.insert(dst, src);
        orbited.entry(src).or_insert_with(|| Vec::new()).push(dst);
    }

    let mut roots = Vec::new();
    for obj in orbited.keys() {
        if !orbits.contains_key(obj) {
            roots.push(*obj);
        }
    }
    println!("roots {:?}", roots);

    let mut total = 0;
    let (mut rank, mut objects) = (1, Vec::new());
    while !orbited.is_empty() {
        println!("rank {}", rank);
        for cur in roots.drain(..) {
            println!("cur {}", cur);
            let children = match orbited.remove(cur) {
                Some(x) => x,
                None => continue,
            };

            for orbiting in children {
                total += rank;
                objects.push(orbiting);
            }
        }

        rank += 1;
        mem::swap(&mut objects, &mut roots);
    }

    println!("total {:?}", total);
    Ok(())
}
