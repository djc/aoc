use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let raw = read_to_string("input/8")?
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    part_2(raw);
    Ok(())
}

#[allow(dead_code)]
fn part_1(raw: Vec<u32>) {
    let (mut fewest_zero, mut out) = (std::usize::MAX, 0);
    for i in 0..(raw.len() / 150) {
        let layer = &raw[i * 150..(i + 1) * 150];
        let zeroes = layer.iter().filter(|v| **v == 0).count();
        if zeroes < fewest_zero {
            let ones = layer.iter().filter(|v| **v == 1).count();
            let twos = layer.iter().filter(|v| **v == 2).count();
            fewest_zero = zeroes;
            out = ones * twos;
        }
    }

    println!("out {}", out);
}

fn part_2(raw: Vec<u32>) {
    let layers = (0..raw.len() / 150)
        .map(|i| &raw[i * 150..(i + 1) * 150])
        .collect::<Vec<_>>();
    for row in 0..6 {
        for col in 0..25 {
            let i = row * 25 + col;
            let p = layers
                .iter()
                .map(|layer| layer[i])
                .find(|v| *v != 2)
                .unwrap();
            print!("{}", p);
        }
        println!("");
    }
}
