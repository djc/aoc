fn main() -> std::io::Result<()> {
    let found = (273025usize..767253usize).filter(|n| validate(*n)).count();
    println!("found: {}", found);
    Ok(())
}

fn validate(n: usize) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    assert_eq!(bytes.len(), 6);
    //println!("validate {:?}", s);
    let pairs = (&bytes[..5]).iter().zip(&bytes[1..]);
    let (mut pair, mut streak) = (false, 0);
    for (x, y) in pairs {
        //println!("pair {} {}", x, y);
        if x == y {
            if streak == 0 {
                streak = 2;
            } else {
                streak += 1;
            }
        } else {
            if streak == 2 {
                pair = true;
            }
            streak = 0;
        }

        if y < x {
            return false;
        }
        //println!("pair {} {}", x, y);
    }

    //println!("same {}", same);
    pair || streak == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(!validate(111111));
        assert!(!validate(223450));
        assert!(!validate(123789));
        assert!(validate(345578));
        assert!(validate(112233));
        assert!(!validate(123444));
        assert!(validate(111122));
    }
}
