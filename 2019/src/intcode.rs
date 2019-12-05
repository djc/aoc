#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub data: Vec<isize>,
    pub pc: usize,
}

impl State {
    pub fn run(&mut self) {
        loop {
            match self.data.get(self.pc) {
                Some(1) => {
                    let left_addr = self.data[self.pc + 1] as usize;
                    let left = self.data[left_addr];
                    let right_addr = self.data[self.pc + 2] as usize;
                    let right = self.data[right_addr];
                    let dst_addr = self.data[self.pc + 3] as usize;
                    self.data[dst_addr] = left + right;
                    self.pc += 4;
                }
                Some(2) => {
                    let left_addr = self.data[self.pc + 1] as usize;
                    let left = self.data[left_addr];
                    let right_addr = self.data[self.pc + 2] as usize;
                    let right = self.data[right_addr];
                    let dst_addr = self.data[self.pc + 3] as usize;
                    self.data[dst_addr] = left * right;
                    self.pc += 4;
                }
                Some(99) => return,
                v => panic!("invalid instruction {:?} at {}", v, self.pc),
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
            pc: 0,
        };
        state.run();
        assert_eq!(state.data, vec![2, 0, 0, 0, 99]);

        let mut state = State {
            data: vec![2, 3, 0, 3, 99],
            pc: 0,
        };
        state.run();
        assert_eq!(state.data, vec![2, 3, 0, 6, 99]);

        let mut state = State {
            data: vec![2, 4, 4, 5, 99, 0],
            pc: 0,
        };
        state.run();
        assert_eq!(state.data, vec![2, 4, 4, 5, 99, 9801]);

        let mut state = State {
            data: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            pc: 0,
        };
        state.run();
        assert_eq!(state.data, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
