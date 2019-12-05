use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let data = read_to_string("input/2")?
        .trim()
        .split(',')
        .map(|s| usize::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let initial = State { data };

    let mut solved = None;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut state = initial.clone();
            state.data[1] = noun;
            state.data[2] = verb;
            state.run(0);
            //println!("{} {} => {}", noun, verb, state.data[0]);
            if state.data[0] == 19690720 {
                println!("solved with {} {}", noun, verb);
                solved = Some((noun, verb));
                break
            }
        }
    }

    if let Some((noun, verb)) = solved {
        println!("calculated {}", 100 * noun + verb);
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    data: Vec<usize>,
}

impl State {
    fn run(&mut self, mut pos: usize) {
        loop {
            match self.data.get(pos) {
                Some(1) => {
                    let left_addr = *self.data.get(pos + 1).unwrap();
                    let left = *self.data.get(left_addr).unwrap();
                    let right_addr = *self.data.get(pos + 2).unwrap();
                    let right = *self.data.get(right_addr).unwrap();
                    let dst_addr = *self.data.get(pos + 3).unwrap();
                    let dst = self.data.get_mut(dst_addr).unwrap();
                    *dst = left + right;
                    pos += 4;
                }
                Some(2) => {
                    let left_addr = *self.data.get(pos + 1).unwrap();
                    let left = *self.data.get(left_addr).unwrap();
                    let right_addr = *self.data.get(pos + 2).unwrap();
                    let right = *self.data.get(right_addr).unwrap();
                    let dst_addr = *self.data.get(pos + 3).unwrap();
                    let dst = self.data.get_mut(dst_addr).unwrap();
                    *dst = left * right;
                    pos += 4;
                }
                Some(99) => return,
                v => panic!("invalid instruction {:?} at {}", v, pos),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut state = State {
            data: vec![1, 0, 0, 0, 99],
        };
        state.run(0);
        assert_eq!(state.data, vec![2, 0, 0, 0, 99]);

        let mut state = State {
            data: vec![2, 3, 0, 3, 99],
        };
        state.run(0);
        assert_eq!(state.data, vec![2, 3, 0, 6, 99]);

        let mut state = State {
            data: vec![2, 4, 4, 5, 99, 0],
        };
        state.run(0);
        assert_eq!(state.data, vec![2, 4, 4, 5, 99, 9801]);

        let mut state = State {
            data: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        };
        state.run(0);
        assert_eq!(state.data, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
