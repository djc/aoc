use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

fn main() -> std::io::Result<()> {
    let map = Map::from_str(&fs::read_to_string("input/10")?);
    let station = part_1(&map).0;
    let vectors = part_2(station, &map);
    list(&vectors);
    Ok(())
}

fn list(vectors: &[Vector]) {
    for (i, v) in vectors.iter().enumerate() {
        println!(
            "i {:04} a {:2},{:2} vx {:03} vy {:03} q {}",
            i,
            v.asteroid.x,
            v.asteroid.y,
            v.x,
            v.y,
            v.angle().quarter()
        );
    }
}

fn part_1(map: &Map) -> (Point, HashSet<Angle>) {
    let mut visible = HashSet::new();
    let mut max = (Point { x: 0, y: 0 }, HashSet::new());
    for origin in map.asteroids.iter().copied() {
        visible.clear();
        for asteroid in map.asteroids.iter().copied() {
            if asteroid == origin {
                continue;
            }
            visible.insert(Vector::new(asteroid, origin).angle());
        }

        if visible.len() > max.1.len() {
            max = (origin, visible.clone());
        }
    }
    max
}

fn part_2(station: Point, map: &Map) -> Vec<Vector> {
    let mut vectors = Vec::new();
    for a in map.asteroids.iter().copied() {
        if station == a {
            continue;
        }
        vectors.push(Vector::new(a, station));
    }

    vectors.sort();
    let mut cur = Angle { x: 0, y: 0 };
    let mut grouped = Vec::new();
    let mut len = vectors.len();
    while len > 0 {
        vectors.retain(|v| {
            if v.angle() == cur {
                true
            } else {
                grouped.push(v.clone());
                cur = v.angle();
                false
            }
        });
        len = vectors.len();
        cur = Angle { x: 0, y: 0 };
    }

    grouped
}

struct Map {
    asteroids: Vec<Point>,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let mut asteroids = Vec::new();
        for (y, ln) in s.trim().split('\n').enumerate() {
            for (x, c) in ln.trim().as_bytes().iter().enumerate() {
                match c {
                    b'.' => {}
                    b'#' => asteroids.push(Point { x, y }),
                    p => panic!("invalid character {}", p),
                }
            }
        }
        Self { asteroids }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Vector {
    x: isize,
    y: isize,
    dist: usize,
    asteroid: Point,
}

impl Vector {
    fn new(asteroid: Point, origin: Point) -> Self {
        let (x, y) = (
            asteroid.x as isize - origin.x as isize,
            origin.y as isize - asteroid.y as isize,
        );

        let dist = (x.abs() + y.abs()) as usize;
        let gcd = gcd(x.abs(), y.abs());
        let (x, y) = (x / gcd, y / gcd);
        Self {
            x,
            y,
            dist,
            asteroid,
        }
    }

    fn angle(&self) -> Angle {
        Angle {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.angle(), self.dist).partial_cmp(&(other.angle(), other.dist))
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.angle(), self.dist).cmp(&(other.angle(), other.dist))
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Angle {
    x: isize,
    y: isize,
}

impl Angle {
    fn quarter(&self) -> u8 {
        match (self.x >= 0, self.y >= 0) {
            (true, true) => 0,
            (true, false) => 1,
            (false, false) => 2,
            (false, true) => 3,
        }
    }

    fn keys(&self, other: &Self) -> (isize, isize) {
        if (self.x >= 0) == (self.y >= 0) {
            if self.y == 0 {
                return (std::isize::MAX, 0);
            } else if other.y == 0 {
                return (0, std::isize::MAX);
            }

            let gcd = gcd(self.y.abs(), other.y.abs());
            let lcm = (self.y.abs() * other.y.abs()) / gcd;
            let (ya, yb) = (lcm / self.y.abs(), lcm / other.y.abs());
            (self.x.abs() * ya, other.x.abs() * yb)
        } else {
            if self.x == 0 {
                return (std::isize::MAX, 0);
            } else if other.x == 0 {
                return (0, std::isize::MAX);
            }

            let gcd = gcd(self.x.abs(), other.x.abs());
            let lcm = (self.x.abs() * other.x.abs()) / gcd;
            let (xa, xb) = (lcm / self.x.abs(), lcm / other.x.abs());
            (self.y.abs() * xa, other.y.abs() * xb)
        }
    }
}

impl PartialOrd for Angle {
    /// UR quarter: x >= 0 (lowest first), y >= 0 (highest first)
    /// LR quarter: x >= 0 (highest first), y < 0 (highest first)
    /// LL quarter: x < 0 (lowest first), y < 0 (highest first)
    /// UL quarter: x < 0 (highest first), y > 0 (lowest first)
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (ka, kb) = self.keys(other);
        (self.quarter(), ka).partial_cmp(&(other.quarter(), kb))
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn gcd(x: isize, y: isize) -> isize {
    let (mut gcd, mut div) = if x > y { (x, y) } else { (y, x) };
    while div != 0 {
        let r = gcd % div;
        gcd = div;
        div = r;
    }
    gcd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
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
        let max = part_1(&map);
        println!("max {:?} {}", max.0, max.1.len());
        assert_eq!(max.0, Point { x: 5, y: 8 });
        assert_eq!(max.1.len(), 33);
    }

    #[test]
    fn part_2_test() {
        let t2 = r"
        .#....#####...#..
        ##...##.#####..##
        ##...#...#.#####.
        ..#.....#...###..
        ..#.#.....#....##
        ";
        let map = Map::from_str(t2);
        let station = part_1(&map).0;
        let vectors = part_2(station, &map);
        list(&vectors);
    }
}
