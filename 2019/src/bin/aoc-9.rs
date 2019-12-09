use aoc_2019::intcode::{State, Status};

fn main() -> std::io::Result<()> {
    let state = State::from_file("input/9");
    part_2(state);
    Ok(())
}

#[allow(dead_code)]
fn part_1(mut state: State) {
    state.input.push_back(1);
    assert_eq!(state.run(), Status::Done);
    println!("out {:?}", state.output);
}

fn part_2(mut state: State) {
    state.input.push_back(2);
    assert_eq!(state.run(), Status::Done);
    println!("out {:?}", state.output);
}
