use aoc_2019::intcode::{State, Status};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let state = State::from_file("input/11");
    let mut robot = Robot::new(500);
    println!("painted {}", robot.run(state));
    Ok(())
}

struct Robot {
    size: usize,
    panels: Vec<u8>,
    pos: (isize, isize),
    dir: Dir,
}

impl Robot {
    fn new(size: usize) -> Self {
        Robot {
            size,
            panels: vec![0; size * size],
            pos: ((size / 2) as isize, (size / 2) as isize),
            dir: Dir::Up,
        }
    }

    fn run(&mut self, mut state: State) -> usize {
        let mut painted = HashSet::new();
        let mut status = Status::NeedInput;
        while status == Status::NeedInput {
            // Provide input from camera
            let color = self.get(self.pos);
            state.input.push_back(color as isize);
            status = state.run();

            // Get color to paint
            let color = state.output.pop_front().unwrap();
            assert!(color >= 0 && color < 2);
            self.set(self.pos, color as u8);
            painted.insert(self.pos);

            // Get turn code
            let turn = state.output.pop_front().unwrap();
            assert!(turn >= 0 && turn < 2);
            self.dir.turn(turn);

            // Move forward by one panel
            self.pos = match self.dir {
                Dir::Up => (self.pos.0, self.pos.1 + 1),
                Dir::Right => (self.pos.0 + 1, self.pos.1),
                Dir::Down => (self.pos.0, self.pos.1 - 1),
                Dir::Left => (self.pos.0 - 1, self.pos.1),
            };
            println!("pos {:?}", self.pos);
        }

        painted.len()
    }

    fn get(&self, pos: (isize, isize)) -> u8 {
        self.panels[dbg!(self.index(pos))]
    }

    fn set(&mut self, pos: (isize, isize), color: u8) {
        let idx = dbg!(self.index(pos));
        self.panels[idx] = color;
    }

    fn index(&self, pos: (isize, isize)) -> usize {
        (pos.0 as usize + pos.1 as usize * self.size)
    }
}

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn(&mut self, code: isize) {
        use Dir::*;
        *self = if code == 1 {
            match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            }
        } else if code == 0 {
            match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            }
        } else {
            panic!("invalid turn code {}", code)
        }
    }
}
