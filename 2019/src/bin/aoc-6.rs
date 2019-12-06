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

    let me = dbg!(path("YOU", &orbits));
    let santa = dbg!(path("SAN", &orbits));
    let mut common = 0;
    while me[me.len() - common - 1] == santa[santa.len() - common - 1] {
        common += 1;
    }

    println!("common {}", common);
    println!("me {}", me.len());
    println!("santa {}", santa.len());
    println!(
        "transfers {}",
        me.len() - common - 1 + santa.len() - common - 1
    );
    Ok(())
}

fn path<'a>(dst: &'a str, orbits: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut path = vec![dst];
    while path.last().unwrap() != &"COM" {
        path.push(orbits.get(path.last().unwrap()).unwrap());
    }
    path
}
