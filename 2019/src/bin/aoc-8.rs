use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let raw = read_to_string("input/8")?
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    
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
    
    Ok(())
}
