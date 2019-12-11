use aoc_2019::intcode::{State, Status};
use permutohedron::heap_recursive;

fn main() -> std::io::Result<()> {
    let state = State::from_file("input/7");
    println!("output {:?}", run2(state));
    Ok(())
}

#[allow(dead_code)]
fn run(state: State) -> isize {
    let mut phases = [0isize, 1, 2, 3, 4];
    let mut max = 0;

    heap_recursive(&mut phases, |permutation| {
        let state = state.clone();
        let mut out = 0;
        //println!("permutation {:?}", permutation);
        for phase in permutation.iter() {
            let mut amp = state.clone();
            amp.input.push_back(*phase);
            amp.input.push_back(out);
            amp.run();
            out = amp.output[0];
        }
        if out > max {
            max = out;
        }
    });

    max
}

fn run2(state: State) -> isize {
    let mut phases = [5isize, 6, 7, 8, 9];
    let mut max = 0;

    heap_recursive(&mut phases, |permutation| {
        let state = state.clone();
        let mut states = vec![];
        for i in 0..5 {
            let mut amp = state.clone();
            amp.input.push_back(permutation[i]);
            states.push(amp);
        }

        let mut out = 0;
        for i in (0..5).cycle() {
            let amp = &mut states[dbg!(i)];
            amp.input.push_back(out);
            let status = amp.run();

            out = amp.output.pop_front().unwrap();
            if status == Status::Done && i == 4 {
                break;
            }
        }

        if out > max {
            max = out;
        }
    });

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let state = State::new(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        assert_eq!(run(state), 43210);

        let state = State::new(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        assert_eq!(run(state), 54321);

        let state = State::new(vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
        assert_eq!(run(state), 65210);
    }

    #[test]
    fn day2() {
        let state = State::new(vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);
        assert_eq!(run2(state), 139629729);

        let state = State::new(vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);
        assert_eq!(run2(state), 18216);
    }
}
