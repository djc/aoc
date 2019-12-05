use std::collections::VecDeque;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub data: Vec<isize>,
    pub pc: usize,
    pub input: VecDeque<isize>,
}

impl State {
    pub fn new(data: Vec<isize>) -> Self {
        Self {
            data,
            pc: 0,
            input: VecDeque::new(),
        }
    }

    pub fn from_file(name: &str) -> Self {
        let data = read_to_string(name)
            .unwrap()
            .trim()
            .split(',')
            .map(|s| isize::from_str(s).unwrap())
            .collect::<Vec<_>>();
        Self::new(data)
    }

    pub fn run(&mut self) {
        loop {
            let instr = self.data[self.pc];
            let (modes, instr) = (instr / 100, instr % 100);
            match instr {
                1 => {
                    let a1 = self.operand(modes, 1);
                    let a2 = self.operand(modes, 2);
                    let dst_addr = self.data[self.pc + 3] as usize;
                    self.data[dst_addr] = a1 + a2;
                    self.pc += 4;
                }
                2 => {
                    let a1 = self.operand(modes, 1);
                    let a2 = self.operand(modes, 2);
                    let dst_addr = self.data[self.pc + 3] as usize;
                    self.data[dst_addr] = a1 * a2;
                    self.pc += 4;
                }
                3 => {
                    let dst_addr = self.data[self.pc + 1] as usize;
                    self.data[dst_addr] = self.input.pop_front().unwrap();
                    self.pc += 2;
                }
                4 => {
                    println!("output: {}", self.operand(modes, 1));
                    self.pc += 2;
                }
                99 => return,
                v => panic!("invalid instruction {:?} at {}", v, self.pc),
            }
        }
    }

    fn operand(&self, modes: isize, idx: usize) -> isize {
        let value = self.data[self.pc + idx];
        let div = 10usize.pow(idx as u32 - 1);
        match (modes as usize / div) % 10 {
            0 => self.data[value as usize],
            1 => value,
            m => panic!("unknown mode {}", m),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut state = State::new(vec![1, 0, 0, 0, 99]);
        state.run();
        assert_eq!(state.data, vec![2, 0, 0, 0, 99]);

        let mut state = State::new(vec![2, 3, 0, 3, 99]);
        state.run();
        assert_eq!(state.data, vec![2, 3, 0, 6, 99]);

        let mut state = State::new(vec![2, 4, 4, 5, 99, 0]);
        state.run();
        assert_eq!(state.data, vec![2, 4, 4, 5, 99, 9801]);

        let mut state = State::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        state.run();
        assert_eq!(state.data, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn day_5() {
        let mut state = State::new(vec![1002, 4, 3, 4, 33]);
        state.run();
        assert_eq!(state.data, vec![1002, 4, 3, 4, 99]);
    }
}