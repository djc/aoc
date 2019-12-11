use aoc_2019::intcode::{State, Status};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut state = State::from_file("input/11");
    let mut robot = Robot::new(250);

    let mut painted = HashSet::new();
    let mut panels = HashSet::new();
    panels.insert(robot.pos);
    let mut status = Status::NeedInput;
    while status == Status::NeedInput {
        painted.insert(robot.pos);

        // Provide input from camera
        let color = match panels.contains(&robot.pos) {
            true => 1,
            false => 0,
        };
        state.input.push_back(color as isize);
        status = state.run();

        let color = state.output.pop_front().unwrap();
        assert!(color >= 0 && color < 2);
        if color == 1 {
            panels.insert(robot.pos);
        } else {
            panels.remove(&robot.pos);
        }

        let turn = state.output.pop_front().unwrap();
        robot.update(turn);
        println!(
            "paint {} turn {} position: {:?} dir {:?}",
            color, turn, robot.pos, robot.dir
        );
        println!("{}", to_string(&panels));
    }

    println!("painted {}", painted.len());
    Ok(())
}

struct Robot {
    pos: (usize, usize),
    dir: Dir,
}

impl Robot {
    fn new(size: usize) -> Self {
        Robot {
            pos: (size, size),
            dir: Dir::Up,
        }
    }

    fn update(&mut self, turn: isize) {
        // Get turn code
        assert!(turn >= 0 && turn < 2);
        self.dir.turn(turn);

        // Move forward by one panel
        self.pos = match self.dir {
            Dir::Up => (self.pos.0, self.pos.1 + 1),
            Dir::Right => (self.pos.0 + 1, self.pos.1),
            Dir::Down => (self.pos.0, self.pos.1 - 1),
            Dir::Left => (self.pos.0 - 1, self.pos.1),
        };
    }
}

fn to_string(panels: &HashSet<(usize, usize)>) -> String {
    let mut s = String::new();
    // cheat and provide a fixed bounding box
    for row in (245..=250).rev() {
        for col in 250..=292 {
            if panels.contains(&(col, row)) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    s
}

#[derive(Debug)]
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
