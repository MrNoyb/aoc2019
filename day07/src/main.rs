use day07::icm::Processor;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::Read;
use std::io::Cursor;

fn parse_input(s: &str) -> Vec<i32> {
    let mut v = vec![];
    for n in s.split(',') {
        let n = match n.trim().parse::<i32>() {
            Err(e) => panic!("Could not parse {}: {}", n, e),
            Ok(i) => i,
        };
        v.push(n);
    }
    return v;
}

fn main() {
    println!("--- Day 7: Amplification Ciruit ---\n");

    println!("Reading input...");

    let mut input = String::new();
    {
        let mut file = File::open("input").expect("Could not open input file.");
        file.read_to_string(&mut input)
            .expect("Could not read from input file.");
    }

    // let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    // let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    // let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

    println!("Parsing input...");
    let memory = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    let mut signals = vec![];

    let perms = (0..5).permutations(5);
    for p in perms {
        // get signal codes
        let sc1 = format!("{}\n", p[0]);
        let sc2 = format!("{}\n", p[1]);
        let sc3 = format!("{}\n", p[2]);
        let sc4 = format!("{}\n", p[3]);
        let sc5 = format!("{}\n", p[4]);

        // create io for processors
        let mut io_a: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));
        let mut io_b: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));
        let mut io_c: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));
        let mut io_d: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));
        let mut io_e: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));

        // prepare io for procA
        io_a.0.get_mut().clear();
        io_a.0.get_mut().extend_from_slice(sc1.as_bytes());
        io_a.0.get_mut().extend_from_slice(&[48, 10]);
        io_a.1.get_mut().clear();

        let mut proc_a = Processor::new(0, memory.clone(), &mut io_a.0, &mut io_a.1);

        proc_a.run();

        // prepare io for procB
        io_b.0.get_mut().clear();
        io_b.0.get_mut().extend_from_slice(sc2.as_bytes());
        io_b.1.get_mut().clear();

        // append output from proc_a
        let mut tmp = io_a
            .1
            .get_ref()
            .iter()
            .copied()
            .take_while(|x| *x != 0)
            .collect();
        io_b.0.get_mut().append(&mut tmp);

        let mut proc_b = Processor::new(0, memory.clone(), &mut io_b.0, &mut io_b.1);

        proc_b.run();

        // prepare io for procC
        io_c.0.get_mut().clear();
        io_c.0.get_mut().extend_from_slice(sc3.as_bytes());
        io_c.1.get_mut().clear();

        // append output from proc_b
        let mut tmp = io_b
            .1
            .get_ref()
            .iter()
            .copied()
            .take_while(|x| *x != 0)
            .collect();
        io_c.0.get_mut().append(&mut tmp);

        let mut proc_c = Processor::new(0, memory.clone(), &mut io_c.0, &mut io_c.1);

        proc_c.run();

        // prepare io for procD
        io_d.0.get_mut().clear();
        io_d.0.get_mut().extend_from_slice(sc4.as_bytes());
        io_d.1.get_mut().clear();

        // append output from proc_c
        let mut tmp = io_c
            .1
            .get_ref()
            .iter()
            .copied()
            .take_while(|x| *x != 0)
            .collect();
        io_d.0.get_mut().append(&mut tmp);

        let mut proc_d = Processor::new(0, memory.clone(), &mut io_d.0, &mut io_d.1);

        proc_d.run();

        // prepare io for procE
        io_e.0.get_mut().clear();
        io_e.0.get_mut().extend_from_slice(sc5.as_bytes());
        io_e.1.get_mut().clear();

        // append output from proc_d
        let mut tmp = io_d
            .1
            .get_ref()
            .iter()
            .copied()
            .take_while(|x| *x != 0)
            .collect();
        io_e.0.get_mut().append(&mut tmp);

        let mut proc_e = Processor::new(0, memory.clone(), &mut io_e.0, &mut io_e.1);

        proc_e.run();

        let v = io_e.1.get_ref().to_vec();
        let err = "ERROR".to_string();
        let signal_e = String::from_utf8(v).unwrap_or(err);
        let sig = signal_e
            .trim_matches('\u{0}')
            .trim()
            .parse::<i32>()
            .expect("Conversion error on signal");
        signals.push(sig);
    }

    println!(
        "Max Thruster Signal: {}",
        signals.iter().max().unwrap_or(&-1)
    );

    println!("\n--- Part 2: ---\n");

    let mut signals = vec![];

    let perms = (5..10).permutations(5);
    for p in perms {
        // get signal codes
        let sc1 = format!("{}\n", p[0]);
        let sc2 = format!("{}\n", p[1]);
        let sc3 = format!("{}\n", p[2]);
        let sc4 = format!("{}\n", p[3]);
        let sc5 = format!("{}\n", p[4]);

        // create io for processors
        let mut io_a: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));
        let mut io_b: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));
        let mut io_c: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));
        let mut io_d: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));
        let mut io_e: (Cursor<Vec<u8>>, Cursor<Vec<u8>>) =
            (Cursor::new(vec![]), Cursor::new(vec![]));

        // prepare io for procA
        io_a.0.get_mut().clear();
        io_a.0.get_mut().extend_from_slice(sc1.as_bytes());
        io_a.0.get_mut().extend_from_slice(&[48, 10]);
        io_a.1.get_mut().clear();

        let mut proc_a = Processor::new(0, memory.clone(), &mut io_a.0, &mut io_a.1);

        proc_a.run();

        // prepare io for procB
        io_b.0.get_mut().clear();
        io_b.0.get_mut().extend_from_slice(sc2.as_bytes());
        io_b.1.get_mut().clear();

        // append output from proc_a
        let mut tmp = io_a
            .1
            .get_ref()
            .iter()
            .copied()
            .take_while(|x| *x != 0)
            .collect();
        io_b.0.get_mut().append(&mut tmp);

        let mut proc_b = Processor::new(0, memory.clone(), &mut io_b.0, &mut io_b.1);

        proc_b.run();

        // prepare io for procC
        io_c.0.get_mut().clear();
        io_c.0.get_mut().extend_from_slice(sc3.as_bytes());
        io_c.1.get_mut().clear();

        // append output from proc_b
        let mut tmp = io_b
            .1
            .get_ref()
            .iter()
            .copied()
            .take_while(|x| *x != 0)
            .collect();
        io_c.0.get_mut().append(&mut tmp);

        let mut proc_c = Processor::new(0, memory.clone(), &mut io_c.0, &mut io_c.1);

        proc_c.run();

        // prepare io for procD
        io_d.0.get_mut().clear();
        io_d.0.get_mut().extend_from_slice(sc4.as_bytes());
        io_d.1.get_mut().clear();

        // append output from proc_c
        let mut tmp = io_c
            .1
            .get_ref()
            .iter()
            .copied()
            .take_while(|x| *x != 0)
            .collect();
        io_d.0.get_mut().append(&mut tmp);

        let mut proc_d = Processor::new(0, memory.clone(), &mut io_d.0, &mut io_d.1);

        proc_d.run();

        // prepare io for procE
        io_e.0.get_mut().clear();
        io_e.0.get_mut().extend_from_slice(sc5.as_bytes());
        io_e.1.get_mut().clear();

        // append output from proc_d
        let mut tmp = io_d
            .1
            .get_ref()
            .iter()
            .copied()
            .take_while(|x| *x != 0)
            .collect();
        io_e.0.get_mut().append(&mut tmp);

        let mut proc_e = Processor::new(0, memory.clone(), &mut io_e.0, &mut io_e.1);

        proc_e.run();

        let v = io_e.1.get_ref().to_vec();
        let err = "ERROR".to_string();
        let signal_e = String::from_utf8(v).unwrap_or(err);
        let sig = signal_e
            .trim_matches('\u{0}')
            .trim()
            .parse::<i32>()
            .expect("Conversion error on signal");
        signals.push(sig);
    }
}

#[test]
fn test() {
    let s1 = "12428642\n\u{0}\u{0}\u{0}\u{0}".to_string();
    let s2 = "12428642\n".to_string();

    assert_eq!(s1.trim_matches('\u{0}'), s2)
}
