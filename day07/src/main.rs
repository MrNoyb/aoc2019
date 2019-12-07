use day07::icm::Processor;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::Read;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
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

    println!("Parsing input...");
    let memory = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    // create io channels
    let (send_a, recv_a) = channel();
    let (send_b, recv_b) = channel();
    let (send_c, recv_c) = channel();
    let (send_d, recv_d) = channel();
    let (send_e, recv_e) = channel();
    let (send_out, recv_out) = channel();

    // create processors
    let proc_a = Arc::new(Mutex::new(Processor::new(
        0,
        memory.clone(),
        recv_a,
        send_b.clone(),
    )));
    let proc_b = Arc::new(Mutex::new(Processor::new(
        0,
        memory.clone(),
        recv_b,
        send_c.clone(),
    )));
    let proc_c = Arc::new(Mutex::new(Processor::new(
        0,
        memory.clone(),
        recv_c,
        send_d.clone(),
    )));
    let proc_d = Arc::new(Mutex::new(Processor::new(
        0,
        memory.clone(),
        recv_d,
        send_e.clone(),
    )));
    let proc_e = Arc::new(Mutex::new(Processor::new(
        0,
        memory.clone(),
        recv_e,
        send_out.clone(),
    )));

    let procs = vec![proc_a, proc_b, proc_c, proc_d, proc_e];

    let mut signals = vec![];

    let perms = (0..5).permutations(5);
    for p in perms {
        // send phase setting sequence
        send_a.send(p[0]).expect("Send error.");
        send_b.send(p[1]).expect("Send error.");
        send_c.send(p[2]).expect("Send error.");
        send_d.send(p[3]).expect("Send error.");
        send_e.send(p[4]).expect("Send error.");

        // send input for proc_a
        send_a.send(0).expect("Send error.");

        // start processors
        let mut vthr = vec![];
        for pu in &procs {
            let pr = Arc::clone(&pu);
            let thr = thread::spawn(move || {
                let mut proc = pr.lock().unwrap();
                proc.run();
            });
            vthr.push(thr);
        }

        // wait for processors to finish
        for thr in vthr {
            thr.join().expect("Thread error");
        }

        let res = recv_out.recv().expect("Could not receive output value");
        signals.push(res);

        // reset processors
        for pu in &procs {
            let mut proc = pu.lock().unwrap();
            proc.set_ip(0);
            proc.set_memory(memory.clone());
        }
    }

    println!(
        "Max Thruster Signal: {}",
        signals.iter().max().unwrap_or(&-1)
    );

    println!("\n--- Part 2: ---\n");

    {
        // rewire feedback loop
        let mut proc_e = procs[4].lock().unwrap();
        proc_e.set_output(send_a.clone());
    }

    // clear result vector
    signals.clear();

    let perms = (5..10).permutations(5);
    for p in perms {
        // send phrase setting sequence
        send_a.send(p[0]).expect("Send error.");
        send_b.send(p[1]).expect("Send error.");
        send_c.send(p[2]).expect("Send error.");
        send_d.send(p[3]).expect("Send error.");
        send_e.send(p[4]).expect("Send error.");

        // send input for proc_a
        send_a.send(0).expect("Send error.");

        // start processors
        let mut vthr = vec![];
        for pu in &procs {
            let pr = Arc::clone(&pu);
            let thr = thread::spawn(move || {
                let mut proc = pr.lock().unwrap();
                proc.run();
            });
            vthr.push(thr);
        }

        // wait for processors to finish
        for thr in vthr {
            thr.join().expect("Thread error");
        }

        {
            let proc_a = procs[0].lock().unwrap();
            let res = proc_a
                .get_input()
                .recv()
                .expect("Could not receive output value");
            signals.push(res);
        }

        // reset processors
        for pu in &procs {
            let mut proc = pu.lock().unwrap();
            proc.set_ip(0);
            proc.set_memory(memory.clone());
        }
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
