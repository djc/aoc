use aoc_2019::intcode::State;

fn main() -> std::io::Result<()> {
    let initial = State::from_file("input/2");

    let mut solved = None;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut state = initial.clone();
            state.data[1] = noun;
            state.data[2] = verb;
            state.run();
            //println!("{} {} => {}", noun, verb, state.data[0]);
            if state.data[0] == 19690720 {
                println!("solved with {} {}", noun, verb);
                solved = Some((noun, verb));
                break;
            }
        }
    }

    if let Some((noun, verb)) = solved {
        println!("calculated {}", 100 * noun + verb);
    }
    Ok(())
}
