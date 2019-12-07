use aoc_2019::intcode::State;

fn main() -> std::io::Result<()> {
    let mut state = State::from_file("input/5");
    state.input.push_back(5);
    state.run();
    println!("output {:?}", state.output);
    Ok(())
}
