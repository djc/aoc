use aoc_2019::intcode::{State, Status};
use std::collections::{BTreeMap, VecDeque};
use std::io::{stdin, BufRead};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut state = State::from_file("input/13");
    state.data[0] = 2;
    let mut map = BTreeMap::new();
    let mut status = state.run();
    let mut user = String::new();
    let _stdin = stdin();
    let mut stdin = _stdin.lock();
    let mut score = 0;

    let mut i = 0;
    while status == Status::NeedInput {
        let (paddle, ball) = update(&mut map, &mut state.output, &mut score);
        if paddle < ball {
            state.input.push_back(1);
        } else if paddle > ball {
            state.input.push_back(-1);
        } else {
            state.input.push_back(0);
        }

        status = state.run();
        if i % 1000 == 0 {
            display(&mut map, score);
        }

        i += 1;
    }

    let (paddle, ball) = update(&mut map, &mut state.output, &mut score);
    display(&mut map, score);
    Ok(())
}

fn update(
    map: &mut BTreeMap<(isize, isize), isize>,
    output: &mut VecDeque<isize>,
    score: &mut isize,
) -> (isize, isize) {
    let mut cur = vec![];
    let (mut paddle, mut ball) = (0, 0);
    for val in output.drain(..) {
        cur.push(val);
        if cur.len() == 3 {
            let (x, y, ty) = (cur[0], cur[1], cur[2]);
            if (x, y) == (-1, 0) {
                *score = ty;
            } else {
                if ty == 3 {
                    paddle = x;
                } else if ty == 4 {
                    ball = x;
                }
                map.insert((y, x), ty);
            }
            cur.clear();
        }
    }

    (paddle, ball)
}

fn display(map: &mut BTreeMap<(isize, isize), isize>, score: isize) {
    let blocks = map.values().filter(|v| **v == 2).count();
    println!("blocks {}, score: {}", blocks, score);
    let mut row = std::isize::MAX;
    for ((y, _), ty) in map.iter() {
        if *y != row {
            print!("\n");
            row = *y;
        }
        print!("{}", ty);
    }
    print!("\n");
}
