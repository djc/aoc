use std::cmp::{Ord, Ordering::*};
use std::convert::TryInto;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut system = System::from_str(&read_to_string("input/12")?);
    let initial = system;
    let mut periodic = [None; 3];
    let (mut i, mut left) = (0, 3);

    while left > 0 {
        system.step();

        for dim in 0..3 {
            if periodic[dim].is_none() {
                let mut matched = 0;
                for (cur, init) in system.moons.iter().zip(initial.moons.iter()) {
                    if cur.pos[dim] == init.pos[dim] && cur.vel[dim] == init.vel[dim] {
                        matched += 1;
                    }
                }

                if matched == system.moons.len() {
                    periodic[dim] = Some(i + 1);
                    println!("dim {} periodic at {}", dim, i + 1);
                    left -= 1;
                }
            }
        }

        i += 1;
    }

    println!(
        "periodic {}",
        lcm(
            periodic[0].unwrap(),
            lcm(periodic[1].unwrap(), periodic[2].unwrap())
        )
    );
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct System {
    moons: [Moon; 4],
}

impl System {
    fn from_str(s: &str) -> Self {
        let moons = s
            .trim()
            .split('\n')
            .map(|s| Moon::from_str(s.trim()))
            .collect::<Vec<_>>();
        Self {
            moons: (&*moons).try_into().unwrap(),
        }
    }

    fn step(&mut self) {
        for (x, y) in Pairs::new(self.moons.len()) {
            let (l, r) = self.moons.split_at_mut(y);
            let (x, y) = (&mut l[x], &mut r[0]);

            for dim in 0..3 {
                let delta = match x.pos[dim].cmp(&y.pos[dim]) {
                    Less => 1,
                    Equal => 0,
                    Greater => -1,
                };

                x.vel[dim] += delta;
                y.vel[dim] -= delta;
            }
        }

        for moon in self.moons.iter_mut() {
            moon.update();
        }
    }

    #[allow(dead_code)]
    fn energy(&self) -> usize {
        self.moons.iter().map(|m| m.energy()).sum()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Moon {
    fn from_str(s: &str) -> Self {
        let (mut x, mut y, mut z) = (None, None, None);
        for (i, v) in s.split(',').enumerate() {
            let start = v.as_bytes().iter().position(|c| *c == b'=').unwrap() + 1;
            let end = if i == 2 { v.len() - 1 } else { v.len() };

            let val = isize::from_str(&v[start..end]).unwrap();
            match i {
                0 => {
                    x = Some(val);
                }
                1 => {
                    y = Some(val);
                }
                2 => {
                    z = Some(val);
                }
                _ => unreachable!(),
            }
        }

        Self {
            pos: [x.unwrap(), y.unwrap(), z.unwrap()],
            vel: [0; 3],
        }
    }

    #[allow(dead_code)]
    fn energy(&self) -> usize {
        let pot = self.pos.iter().map(|p| p.abs() as usize).sum::<usize>();
        let kin = self.vel.iter().map(|v| v.abs() as usize).sum::<usize>();
        pot * kin
    }

    fn update(&mut self) {
        for dim in 0..3 {
            self.pos[dim] += self.vel[dim];
        }
    }
}

struct Pairs {
    len: usize,
    base: usize,
    cur: usize,
}

impl Pairs {
    fn new(len: usize) -> Self {
        Self {
            len,
            base: 0,
            cur: 1,
        }
    }
}

impl Iterator for Pairs {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.base >= self.len - 1 {
            return None;
        }

        let (x, y) = (self.base, self.cur);
        if self.cur == self.len - 1 {
            self.base += 1;
            self.cur = self.base + 1;
        } else {
            self.cur += 1;
        }

        Some((x, y))
    }
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

fn gcd(x: usize, y: usize) -> usize {
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
    fn basic() {
        let s = r"
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>
        ";
        let mut system = System::from_str(s);
        for _ in 0..10 {
            system.step();
        }

        assert_eq!(system.energy(), 179);
    }
}
