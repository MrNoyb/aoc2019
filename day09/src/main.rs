use day09::icm::Processor;
use std::fs::File;
use std::io::prelude::Read;
use std::sync::mpsc::channel;

fn parse_input(s: &str) -> Vec<i128> {
    let mut v = vec![];
    for n in s.split(',') {
        let n = match n.trim().parse::<i128>() {
            Err(e) => panic!("Could not parse {}: {}", n, e),
            Ok(i) => i,
        };
        v.push(n);
    }
    return v;
}

fn main() {
    println!("--- Day 9: Sensor Boost ---\n");

    println!("Reading input...");

    let mut input = String::new();
    {
        let mut file = File::open("input").expect("Could not open input file.");
        file.read_to_string(&mut input)
            .expect("Could not read from input file.");
    }

    println!("Parsing input...");
    let memory = parse_input(&input);

    let (to_proc, proc_in) = channel();
    let (proc_out, from_proc) = channel();

    println!("\n--- Part 1: ---\n");

    let mut proc = Processor::new(0, memory.clone(), proc_in, proc_out);

    to_proc.send(1).expect("Could not send input to processor.");

    proc.run();

    println!("BOOST keycode: {}", from_proc.recv().unwrap());

    println!("\n--- Part 2: ---\n");

    proc.reset();
    proc.load_into_memory(&memory);

    to_proc.send(2).expect("Could not send input to processor.");

    proc.run();

    println!("Coordinates: {}", from_proc.recv().unwrap());
}
