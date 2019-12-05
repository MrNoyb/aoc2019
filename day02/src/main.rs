use std::cmp::Eq;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Instruction {
    Unknown,
    Halt,
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Instruction::Unknown, Instruction::Unknown) => true,
            (Instruction::Halt, Instruction::Halt) => true,
            (Instruction::Add(x, y, z), Instruction::Add(a, b, c)) => x == a && y == b && z == c,
            (Instruction::Mul(x, y, z), Instruction::Mul(a, b, c)) => x == a && y == b && z == c,
            (_, _) => false,
        }
    }
}

impl Eq for Instruction {}

fn parse_input(s: &str) -> Vec<usize> {
    let mut v = vec![];
    for n in s.split(',') {
        let n = match n.trim().parse::<usize>() {
            Err(e) => panic!("Could not parse {}: {}", n, e),
            Ok(i) => i,
        };
        v.push(n);
    }
    return v;
}

fn parse_instruction(ip: usize, mem: &[usize]) -> Instruction {
    if let Some(val) = mem.get(ip) {
        match val {
            1 => {
                let (p1, p2) = fetch_params(mem, mem[ip + 1], mem[ip + 2]);
                Instruction::Add(p1, p2, mem[ip + 3])
            }
            2 => {
                let (p1, p2) = fetch_params(mem, mem[ip + 1], mem[ip + 2]);
                Instruction::Mul(p1, p2, mem[ip + 3])
            }
            99 => Instruction::Halt,
            _ => Instruction::Unknown,
        }
    } else {
        panic!("Invalid memory address");
    }
}

fn fetch_params(mem: &[usize], i1: usize, i2: usize) -> (usize, usize) {
    let v1 = mem.get(i1).expect("Invalid memory index.");
    let v2 = mem.get(i2).expect("Invalid memory index.");
    (*v1, *v2)
}

fn run_instr(mem: &mut [usize], op: &Instruction) -> bool {
    match op {
        Instruction::Unknown => panic!("Invalid instruction"),
        Instruction::Halt => return false,
        Instruction::Add(p1, p2, res) => mem[*res] = p1 + p2,
        Instruction::Mul(p1, p2, res) => mem[*res] = p1 * p2,
    }
    return true;
}

fn run_computer(mem: Vec<usize>) -> Vec<usize> {
    let mut ip = 0;
    let mut mem = mem;
    loop {
        if ip >= mem.len() {
            panic!("Invalid IP address.");
        }
        let op = parse_instruction(ip, &mem);
        if !run_instr(&mut mem, &op) {
            break;
        }
        ip += 4;
    }
    return mem;
}

fn main() {
    println!("--- Day 2: 1202 Program Alarm ---\n");

    println!("Reading input from file...");
    let mut input = String::new();

    {
        let mut file = File::open("input").expect("Could not open file.");
        file.read_to_string(&mut input)
            .expect("Could not read file.");
    }

    println!("Parsing input...");
    let input = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    let mut mem1 = input.clone();
    mem1[1] = 12;
    mem1[2] = 2;

    let mem1 = run_computer(mem1);

    println!("Value at position 0: {}", mem1[0]);

    println!("\n--- Part 2: ---\n");

    for noun in 1..=99 {
        for verb in 1..=99 {
            let mut mem = input.clone();
            mem[1] = noun;
            mem[2] = verb;

            let mem = run_computer(mem);

            if mem[0] == 19690720 {
                println!("N/V-Combo: {}", 100 * noun + verb);
                break;
            }
        }
    }
}

#[test]
fn test_p1t1() {
    let input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mem = parse_input(&input);
    let mut exp_mem = mem.clone();
    exp_mem[0] = 3500;
    exp_mem[3] = 70;

    let final_mem = run_computer(mem);

    assert_eq!(exp_mem, final_mem);
}

#[test]
fn test_p1t2() {
    let input = "1,0,0,0,99";
    let mem = parse_input(&input);
    let mut exp_mem = mem.clone();
    exp_mem[0] = 2;

    let final_mem = run_computer(mem);

    assert_eq!(exp_mem, final_mem);
}

#[test]
fn test_p1t3() {
    let input = "2,3,0,3,99";
    let mem = parse_input(&input);
    let mut exp_mem = mem.clone();
    exp_mem[3] = 6;

    let final_mem = run_computer(mem);

    assert_eq!(exp_mem, final_mem);
}

#[test]
fn test_p1t4() {
    let input = "2,4,4,5,99,0";
    let mem = parse_input(&input);
    let mut exp_mem = mem.clone();
    exp_mem[5] = 9801;

    let final_mem = run_computer(mem);

    assert_eq!(exp_mem, final_mem);
}

#[test]
fn test_p1t5() {
    let input = "1,1,1,4,99,5,6,0,99";
    let mem = parse_input(&input);
    let mut exp_mem = mem.clone();
    exp_mem[0] = 30;
    exp_mem[4] = 2;

    println!("Before:");
    println!("Exp: {:?}", exp_mem);
    println!("End: {:?}", mem);

    let final_mem = run_computer(mem);

    println!("After:");
    println!("Exp: {:?}", exp_mem);
    println!("End: {:?}", final_mem);
    assert_eq!(exp_mem, final_mem);
}
