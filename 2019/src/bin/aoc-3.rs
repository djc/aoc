use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let raw = read_to_string("input/3")?;
    let data = raw.trim().split('\n').collect::<Vec<_>>();
    println!("distance: {}", run(&data));
    Ok(())
}

fn run(data: &[&str]) -> isize {
    let data = data
        .iter()
        .map(|s| {
            s.split(',')
                .map(|s| Edge::from_str(s).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sets = vec![];
    for edges in data {
        let mut nodes = HashMap::new();
        let mut pos = Node { x: 0, y: 0 };
        let mut step = 0isize;
        for edge in edges {
            let len = edge.len as isize;
            assert!(len > 0);
            let Node { x, y } = pos;
            for i in 1..(len + 1) {
                pos = match edge.dir {
                    Dir::Up => Node { x, y: y + i },
                    Dir::Right => Node { x: x + i, y },
                    Dir::Down => Node { x, y: y - i },
                    Dir::Left => Node { x: x - i, y },
                };
                step += 1;
                nodes.insert(pos, step);
            }
        }
        sets.push(nodes);
    }

    let mut steps = None;
    for (node, s1) in sets[0].iter() {
        match (sets[1].get(node), &steps) {
            (Some(s2), Some(cur)) => {
                if (s1 + s2) < *cur {
                    steps = Some(s1 + s2);
                }
            }
            (Some(s2), None) => {
                steps = Some(s1 + s2);
            }
            (None, None) | (None, Some(_)) => {}
        }
    }

    steps.unwrap()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Edge {
    dir: Dir,
    len: usize,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Edge, ()> {
        let dir = Dir::from_byte(s[0..1].as_bytes()[0]);
        let len = usize::from_str(&s[1..]).unwrap();
        Ok(Edge { dir, len })
    }
}

#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn from_byte(b: u8) -> Self {
        match b {
            b'U' => Dir::Up,
            b'R' => Dir::Right,
            b'D' => Dir::Down,
            b'L' => Dir::Left,
            _ => panic!("invalid dir {}", b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let data = ["R8,U5,L5,D3", "U7,R6,D4,L4"];
        assert_eq!(run(&data), 30);

        let data = [
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];
        assert_eq!(run(&data), 610);

        let data = [
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        assert_eq!(run(&data), 410);
    }
}
