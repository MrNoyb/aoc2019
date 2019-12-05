use itertools::Itertools;

const L_BOUND: u32 = 264793;
const U_BOUND: u32 = 803935;

fn is_valid_p1(code: u32) -> bool {
    let vcode = code
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    vcode.len() == 6
        && vcode[..].windows(2).all(|w| w[0] <= w[1])
        && vcode[..].windows(2).any(|w| w[0] == w[1])
}

fn is_valid_p2(code: u32) -> bool {
    let vcode = code
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    vcode.len() == 6 && vcode[..].windows(2).all(|w| w[0] <= w[1]) && {
        for (_, g) in &vcode[..].into_iter().group_by(|x| *x) {
            if g.count() == 2 {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("--- Day 4: Secure Container ---\n");

    println!("\n--- Part 1: ---\n");

    let mut count1 = 0;
    let mut count2 = 0;
    for code in L_BOUND..=U_BOUND {
        if is_valid_p1(code) {
            count1 += 1;
        }
        if is_valid_p2(code) {
            count2 += 1;
        }
    }

    println!("Possible key codes: {}", count1);

    println!("\n--- Part 2: ---\n");

    println!("Possible key codes: {}", count2);
}

#[test]
fn test_is_valid_p1() {
    assert!(is_valid_p1(111111));
    assert!(!is_valid_p1(223450));
    assert!(!is_valid_p1(123789));
}

#[test]
fn test_is_valid_p2() {
    assert!(is_valid_p2(112233));
    assert!(!is_valid_p2(123444));
    assert!(is_valid_p2(111122));
}
