use std::collections::HashSet;
use std::fs;

fn main() -> std::io::Result<()> {
    let map = Map::from_str(&fs::read_to_string("input/10")?);
    let max = run(map);
    println!("max {:?}", (max.0, max.1.len()));
    Ok(())
}

fn run(map: Map) -> (Point, HashSet<(isize, isize)>) {
    let mut visible = HashSet::new();
    let mut max = (Point { x: 0, y: 0 }, HashSet::new());
    for origin in map.asteroids.iter().copied() {
        visible.clear();
        for asteroid in map.asteroids.iter().copied() {
            if asteroid == origin {
                continue;
            }
            visible.insert(angle(asteroid, origin));
        }

        if visible.len() > max.1.len() {
            max = (origin, visible.clone());
        }
    }
    max
}

struct Map {
    options: Vec<Point>,
    asteroids: Vec<Point>,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let mut options = Vec::new();
        let mut asteroids = Vec::new();
        for (y, ln) in s.trim().split('\n').enumerate() {
            for (x, c) in ln.trim().as_bytes().iter().enumerate() {
                match c {
                    b'.' => options.push(Point { x, y }),
                    b'#' => asteroids.push(Point { x, y }),
                    p => panic!("invalid character {}", p),
                }
            }
        }
        Self { options, asteroids }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn angle(asteroid: Point, origin: Point) -> (isize, isize) {
    let (x, y) = (
        asteroid.x as isize - origin.x as isize,
        asteroid.y as isize - origin.y as isize,
    );
    //println!("angle {:?} {:?}", asteroid, origin);

    let (gcd, div) = (x.abs(), y.abs());
    let (mut gcd, mut div) = if gcd > div { (gcd, div) } else { (div, gcd) };

    while div != 0 {
        let r = gcd % div;
        gcd = div;
        div = r;
    }

    //println!("gcd {} x {} y {}", gcd, x / gcd, y / gcd);
    (x / gcd, y / gcd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let t1 = r"
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####
        ";
        let map = Map::from_str(t1);
        let max = run(map);
        println!("max {:?} {}", max.0, max.1.len());
        assert_eq!(max.0, Point { x: 5, y: 8 });
        assert_eq!(max.1.len(), 33);
    }
}
