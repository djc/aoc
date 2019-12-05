#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub data: Vec<usize>,
}

impl State {
    pub fn run(&mut self, mut pos: usize) {
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
