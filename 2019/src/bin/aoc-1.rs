use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let f = BufReader::new(File::open("input/1")?);
    let modules = f.lines()
        .map(|ln| recursive_fuel(fuel(u64::from_str(&ln.unwrap()).unwrap())))
        .sum::<u64>();
    println!("fuel required for modules: {}", modules);
    Ok(())
}

fn recursive_fuel(mut mass: u64) -> u64 {
    let mut base = mass;
    loop {
        base = fuel(base);
        mass += base;
        if base < 1 {
            return mass;
        }
    }
}

fn fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn recursive() {
        assert_eq!(recursive_fuel(fuel(14)), 2);
        assert_eq!(recursive_fuel(fuel(1969)), 966);
        assert_eq!(recursive_fuel(fuel(100756)), 50346);
    }
}
