use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub data: Vec<isize>,
    pub pc: usize,
    pub input: VecDeque<isize>,
    pub output: Vec<isize>,
}

impl State {
    pub fn new(data: Vec<isize>) -> Self {
        Self {
            data,
            pc: 0,
            input: VecDeque::new(),
            output: Vec::new(),
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

    pub fn run(&mut self) -> Status {
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
                    if self.input.is_empty() {
                        return Status::NeedInput;
                    }
                    self.data[dst_addr] = self.input.pop_front().unwrap();
                    self.pc += 2;
                }
                4 => {
                    self.output.push(self.operand(modes, 1));
                    self.pc += 2;
                }
                5 => {
                    let a1 = self.operand(modes, 1);
                    let a2 = self.operand(modes, 2);
                    if a1 > 0 {
                        self.pc = a2.try_into().unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    let a1 = self.operand(modes, 1);
                    let a2 = self.operand(modes, 2);
                    if a1 == 0 {
                        self.pc = a2.try_into().unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    let a1 = self.operand(modes, 1);
                    let a2 = self.operand(modes, 2);
                    let dst_addr = self.data[self.pc + 3] as usize;
                    let val = if a1 < a2 { 1 } else { 0 };
                    self.data[dst_addr] = val;
                    self.pc += 4;
                }
                8 => {
                    let a1 = self.operand(modes, 1);
                    let a2 = self.operand(modes, 2);
                    let dst_addr = self.data[self.pc + 3] as usize;
                    let val = if a1 == a2 { 1 } else { 0 };
                    self.data[dst_addr] = val;
                    self.pc += 4;
                }
                99 => return Status::Done,
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

#[derive(Eq, PartialEq)]
pub enum Status {
    NeedInput,
    Done,
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
        let mut state = State::new(vec![3, 0, 4, 0, 99]);
        state.input.push_back(4);
        state.run();
        assert_eq!(state.output, vec![4]);

        let mut state = State::new(vec![1002, 4, 3, 4, 33]);
        state.run();
        assert_eq!(state.data, vec![1002, 4, 3, 4, 99]);

        let mut state = State::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        state.input.push_back(4);
        state.run();
        assert_eq!(state.output, vec![0]);

        let mut state = State::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        state.input.push_back(4);
        state.run();
        assert_eq!(state.output, vec![1]);

        let mut state = State::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        state.input.push_back(8);
        state.run();
        assert_eq!(state.output, vec![1]);

        let mut state = State::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        state.input.push_back(9);
        state.run();
        assert_eq!(state.output, vec![0]);
    }
}
