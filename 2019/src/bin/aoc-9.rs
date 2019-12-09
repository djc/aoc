use aoc_2019::intcode::{State, Status};

fn main() -> std::io::Result<()> {
    let state = State::from_file("input/9");
    part_1(state);
    Ok(())
}

fn part_1(mut state: State) {
    state.input.push_back(1);
    assert_eq!(state.run(), Status::Done);
    println!("out {:?}", state.output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        //
    }
}
