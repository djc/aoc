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
    pub base: isize,
}

impl State {
    pub fn new(data: Vec<isize>) -> Self {
        Self {
            data,
            pc: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            base: 0,
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
                    self.store(modes, 3, a1 + a2);
                    self.pc += 4;
                }
                2 => {
                    let a1 = self.operand(modes, 1);
                    let a2 = self.operand(modes, 2);
                    self.store(modes, 3, a1 * a2);
                    self.pc += 4;
                }
                3 => {
                    if self.input.is_empty() {
                        return Status::NeedInput;
                    }
                    let val = self.input.pop_front().unwrap();
                    assert!(instr < 100);
                    self.store(modes, 1, val);
                    self.pc += 2;
                }
                4 => {
                    let value = self.operand(modes, 1);
                    self.output.push(value);
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
                    self.store(modes, 3, if a1 < a2 { 1 } else { 0 });
                    self.pc += 4;
                }
                8 => {
                    let a1 = self.operand(modes, 1);
                    let a2 = self.operand(modes, 2);
                    self.store(modes, 3, if a1 == a2 { 1 } else { 0 });
                    self.pc += 4;
                }
                9 => {
                    let a1 = self.operand(modes, 1);
                    self.base += a1;
                    self.pc += 2;
                }
                99 => return Status::Done,
                v => panic!("invalid instruction {:?} at {}", v, self.pc),
            }
        }
    }

    fn operand(&mut self, modes: isize, idx: usize) -> isize {
        let value = self.data[self.pc + idx];
        match self.mode(modes, idx) {
            0 => self.get(value),
            1 => value,
            2 => self.get(self.base + value),
            m => panic!("unknown mode {}", m),
        }
    }

    fn store(&mut self, modes: isize, idx: usize, value: isize) {
        let addr = self.data[self.pc + idx];
        let addr = match self.mode(modes, idx) {
            0 => addr,
            2 => self.base + addr,
            m => panic!("invalid store mode {}", m),
        } as usize;

        if addr > self.data.len() - 1 {
            self.data.resize(addr + 1, 0);
        }
        self.data[addr] = value;
    }

    fn mode(&self, modes: isize, idx: usize) -> usize {
        ((modes as usize) / 10usize.pow(idx as u32 - 1)) % 10
    }

    fn get(&mut self, addr: isize) -> isize {
        let addr = addr as usize;
        if addr > self.data.len() {
            self.data.resize(addr + 1, 0);
        }
        self.data[addr]
    }
}

#[derive(Debug, Eq, PartialEq)]
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

    #[test]
    fn day_9() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut state = State::new(code.clone());
        state.run();
        assert_eq!(state.output, code);

        let mut state = State::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        state.run();
        assert_eq!(state.output.pop().unwrap(), 1_219_070_632_396_864);

        let mut state = State::new(vec![104, 1125899906842624, 99]);
        state.run();
        assert_eq!(state.output.pop().unwrap(), 1125899906842624);
    }
}
