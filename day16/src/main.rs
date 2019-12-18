fn nphases(n: u32, signal: &[i32], pattern: &[i32]) -> Vec<i32> {
    let slen = signal.len();
    let mut res = signal.to_vec();

    let mut v = Vec::with_capacity(slen);
    let mut last = 0;
    let mut lneg = 0;
    let mut lpos = 0;
    for _ in 0..n {
        for i in 1..=slen {
            if i > slen / 2 {
                if last != 0 {
                    last -= res[i - 2];
                } else {
                    last = res.iter().skip(i - 1).sum();
                }
                v.push(last.abs() % 10);
            } else if i > slen / 4 {
                if lneg == 0 || lpos == 0 {
                    lneg = res.iter().skip(i * 3 - 1).sum();
                    lpos = res.iter().skip(i - 1).take(i).sum();
                } else {
                    lneg -= res[i * 2 - 2];
                    lpos -= res[i - 2];
                }
                v.push((lpos - lneg).abs() % 10);
            } else {
                let pat = npattern(i, pattern);
                let sit = res.iter().skip(i - 1);
                let pit = pat.iter().cycle().skip(i);
                let s = sit.zip(pit).fold(0, |acc, (x, f)| acc + x * f);
                v.push(s.abs() % 10);
            }
        }
        res.clear();
        res.append(&mut v);
        last = 0;
        lneg = 0;
        lpos = 0;
    }

    return res;
}

// works only because offset is bigger than signal.len() / 2.
fn nphases2(n: u32, signal: &[i32]) -> Vec<i32> {
    let slen = signal.len();
    let mut res = signal.to_vec();

    let mut v = Vec::with_capacity(slen);
    for _ in 0..n {
        let mut sum = 0;
        for i in (0..slen).rev() {
            sum += res[i];
            v.push(sum % 10);
        }
        res.clear();
        res.extend(v.drain(0..).rev());
    }

    return res;
}

fn npattern(n: usize, pattern: &[i32]) -> Vec<i32> {
    if n <= 1 {
        pattern.to_vec()
    } else {
        let mut v = vec![];
        for i in pattern {
            for _ in 0..n {
                v.push(*i);
            }
        }
        v
    }
}

fn parse_input(s: &str) -> Vec<i32> {
    let mut v = Vec::with_capacity(s.len());
    for c in s.chars() {
        let n = c.to_digit(10).expect("Could not parse input");
        v.push(n as i32);
    }
    return v;
}

fn main() {
    println!("--- Day 16: Flawed Frequency Transmission ---\n");

    println!("Reading input...");

    let input = include_str!("../input").trim();

    let pattern = vec![0, 1, 0, -1];

    println!("\n--- Part 1: ---\n");

    let signal = parse_input(&input);
    let phase100 = nphases(100, &signal, &pattern);
    print!("First 8 digits after 100 phases: ");
    phase100.iter().take(8).for_each(|x| print!("{}", x));
    print!("\n");

    println!("\n--- Part 2: ---\n");

    let input = &input.repeat(10000);
    // println!("Real Signal:");
    // let len = input.len();
    // println!("Len: {}", len);

    let offset = input
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .expect("Could not parse offset");

    let signal = parse_input(&input);
    let n = 100;
    let phase100 = nphases2(n, &signal[offset..]);
    print!("First 8 digits after {} phases at offset {}: ", n, offset);
    phase100
        .iter()
        // .skip(offset)
        .take(8)
        .for_each(|x| print!("{}", x));
    print!("\n");
}

#[test]
fn test1() {
    let input = "12345678";
    let signal = parse_input(&input);
    let pattern = vec![0, 1, 0, -1];

    // parsing
    assert_eq!(signal, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    // pattern
    assert_eq!(npattern(1, &pattern), pattern);
    assert_eq!(
        npattern(3, &pattern),
        vec![0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
    );
    // phases
    assert_eq!(nphases(1, &signal, &pattern), vec![4, 8, 2, 2, 6, 1, 5, 8]);
    assert_eq!(nphases(2, &signal, &pattern), vec![3, 4, 0, 4, 0, 4, 3, 8]);
    assert_eq!(nphases(3, &signal, &pattern), vec![0, 3, 4, 1, 5, 5, 1, 8]);
    assert_eq!(nphases(4, &signal, &pattern), vec![0, 1, 0, 2, 9, 4, 9, 8]);

    assert_eq!(nphases2(1, &signal), vec![4, 8, 2, 2, 6, 1, 5, 8]);
    assert_eq!(nphases2(2, &signal), vec![3, 4, 0, 4, 0, 4, 3, 8]);
    assert_eq!(nphases2(3, &signal), vec![0, 3, 4, 1, 5, 5, 1, 8]);
    assert_eq!(nphases2(4, &signal), vec![0, 1, 0, 2, 9, 4, 9, 8]);
}
