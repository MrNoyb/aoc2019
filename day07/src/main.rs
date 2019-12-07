use day07::icm::Processor;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::Read;
use std::sync::mpsc::channel;
use std::thread;

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

    let (send_out, recv_out) = channel();

    let perms = (0..5).permutations(5);
    for p in perms {
        // create io channels
        let (send_a, recv_a) = channel();
        let (send_b, recv_b) = channel();
        let (send_c, recv_c) = channel();
        let (send_d, recv_d) = channel();
        let (send_e, recv_e) = channel();
        // create processors
        let mut proc_a = Processor::new(0, memory.clone(), recv_a, send_b.clone());
        let mut proc_b = Processor::new(0, memory.clone(), recv_b, send_c.clone());
        let mut proc_c = Processor::new(0, memory.clone(), recv_c, send_d.clone());
        let mut proc_d = Processor::new(0, memory.clone(), recv_d, send_e.clone());
        let mut proc_e = Processor::new(0, memory.clone(), recv_e, send_out.clone());

        // send phrase setting sequence
        send_a.send(p[0]).expect("Send error.");
        send_b.send(p[1]).expect("Send error.");
        send_c.send(p[2]).expect("Send error.");
        send_d.send(p[3]).expect("Send error.");
        send_e.send(p[4]).expect("Send error.");

        // send input for proc_a
        send_a.send(0).expect("Send error.");

        let t1 = thread::spawn(move || {
            proc_a.run();
        });
        let t2 = thread::spawn(move || {
            proc_b.run();
        });
        let t3 = thread::spawn(move || {
            proc_c.run();
        });
        let t4 = thread::spawn(move || {
            proc_d.run();
        });
        let t5 = thread::spawn(move || {
            proc_e.run();
        });

        t1.join().expect("Thread error.");
        t2.join().expect("Thread error.");
        t3.join().expect("Thread error.");
        t4.join().expect("Thread error.");
        t5.join().expect("Thread error.");

        let res = recv_out.recv().expect("Could not receive output value");
        signals.push(res);
    }

    println!(
        "Max Thruster Signal: {}",
        signals.iter().max().unwrap_or(&-1)
    );

    println!("\n--- Part 2: ---\n");

    signals.clear();

    let perms = (5..10).permutations(5);
    for p in perms {
        // create io channels
        let (send_a, recv_a) = channel();
        let (send_b, recv_b) = channel();
        let (send_c, recv_c) = channel();
        let (send_d, recv_d) = channel();
        let (send_e, recv_e) = channel();
        // create processors
        let mut proc_a = Processor::new(0, memory.clone(), recv_a, send_b.clone());
        let mut proc_b = Processor::new(0, memory.clone(), recv_b, send_c.clone());
        let mut proc_c = Processor::new(0, memory.clone(), recv_c, send_d.clone());
        let mut proc_d = Processor::new(0, memory.clone(), recv_d, send_e.clone());
        let mut proc_e = Processor::new(0, memory.clone(), recv_e, send_a.clone());

        // send phrase setting sequence
        send_a.send(p[0]).expect("Send error.");
        send_b.send(p[1]).expect("Send error.");
        send_c.send(p[2]).expect("Send error.");
        send_d.send(p[3]).expect("Send error.");
        send_e.send(p[4]).expect("Send error.");

        // send input for proc_a
        send_a.send(0).expect("Send error.");

        // let t1 = thread::spawn(move || {
        //     proc_a.run();
        // });
        let t2 = thread::spawn(move || {
            proc_b.run();
        });
        let t3 = thread::spawn(move || {
            proc_c.run();
        });
        let t4 = thread::spawn(move || {
            proc_d.run();
        });
        let t5 = thread::spawn(move || {
            proc_e.run();
        });

        proc_a.run();
        // t1.join().expect("Thread error.");
        t2.join().expect("Thread error.");
        t3.join().expect("Thread error.");
        t4.join().expect("Thread error.");
        t5.join().expect("Thread error.");

        let res = proc_a
            .get_input()
            .recv()
            .expect("Could not receive output value");
        signals.push(res);
    }

    println!(
        "Max Thruster Signal: {}",
        signals.iter().max().unwrap_or(&-1)
    );
}

#[test]
fn test() {
    let s1 = "12428642\n\u{0}\u{0}\u{0}\u{0}".to_string();
    let s2 = "12428642\n".to_string();

    assert_eq!(s1.trim_matches('\u{0}'), s2)
}
