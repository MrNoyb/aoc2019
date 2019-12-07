use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Param {
    Position(usize),
    Immediate(i32),
}

#[derive(Debug)]
enum Instr {
    Unknown,
    Halt,
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    Store(Param),
    Show(Param),
    JmpT(Param, Param),
    JmpF(Param, Param),
    CmpLt(Param, Param, Param),
    CmpEq(Param, Param, Param),
}

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

/*
Parses a parameter.
nth   - Number of the parameter, beginning by zero.
pcode - Paramter mode code. A number where each digit specifies a parameter mode.
val   - Value of the parameter.
*/
fn parse_param(nth: u32, pcode: i32, val: i32) -> Param {
    if (pcode / 10_i32.pow(nth)) % 10 == 0 {
        let p: usize = val.try_into().unwrap();
        Param::Position(p)
    } else {
        Param::Immediate(val)
    }
}

fn fetch_instruction(ip: usize, mem: &[i32]) -> Instr {
    if let Some(val) = mem.get(ip) {
        let opcode = val % 100;
        let pcode = val / 100;

        match opcode {
            1 => {
                let p0 = parse_param(0, pcode, mem[ip + 1]);
                let p1 = parse_param(1, pcode, mem[ip + 2]);
                let p2 = parse_param(2, pcode, mem[ip + 3]);
                Instr::Add(p0, p1, p2)
            }
            2 => {
                let p0 = parse_param(0, pcode, mem[ip + 1]);
                let p1 = parse_param(1, pcode, mem[ip + 2]);
                let p2 = parse_param(2, pcode, mem[ip + 3]);
                Instr::Mul(p0, p1, p2)
            }
            3 => {
                let p0 = parse_param(0, pcode, mem[ip + 1]);
                Instr::Store(p0)
            }
            4 => {
                let p0 = parse_param(0, pcode, mem[ip + 1]);
                Instr::Show(p0)
            }
            5 => {
                let p0 = parse_param(0, pcode, mem[ip + 1]);
                let p1 = parse_param(1, pcode, mem[ip + 2]);
                Instr::JmpT(p0, p1)
            }
            6 => {
                let p0 = parse_param(0, pcode, mem[ip + 1]);
                let p1 = parse_param(1, pcode, mem[ip + 2]);
                Instr::JmpF(p0, p1)
            }
            7 => {
                let p0 = parse_param(0, pcode, mem[ip + 1]);
                let p1 = parse_param(1, pcode, mem[ip + 2]);
                let p2 = parse_param(2, pcode, mem[ip + 3]);
                Instr::CmpLt(p0, p1, p2)
            }
            8 => {
                let p0 = parse_param(0, pcode, mem[ip + 1]);
                let p1 = parse_param(1, pcode, mem[ip + 2]);
                let p2 = parse_param(2, pcode, mem[ip + 3]);
                Instr::CmpEq(p0, p1, p2)
            }
            99 => Instr::Halt,
            _ => Instr::Unknown,
        }
    } else {
        panic!("Invalid memory address");
    }
}

fn readnum() -> i32 {
    print!("Enter value: ");
    io::stdout().flush().expect("Could not flush stdout.");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<i32>().unwrap(),
        Err(_) => panic!("Could not read from stdin."),
    }
}

fn fetch_param(mem: &[i32], p: Param) -> i32 {
    match p {
        Param::Position(n) => mem[n],
        Param::Immediate(n) => n,
    }
}

fn run_instr(ip: &mut usize, mem: &mut [i32]) -> bool {
    match fetch_instruction(*ip, mem) {
        Instr::Unknown => {
            println!("PANIC: Invalid instruction");
            return false;
        }
        Instr::Halt => {
            println!("HALT");
            return false;
        }
        Instr::Add(p0, p1, p2) => {
            let p0 = fetch_param(mem, p0);
            let p1 = fetch_param(mem, p1);
            let p2 = match p2 {
                Param::Position(n) => n,
                _ => panic!("Invalid parameter"),
            };
            mem[p2] = p0 + p1;
            *ip += 4;
        }
        Instr::Mul(p0, p1, p2) => {
            let p0 = fetch_param(mem, p0);
            let p1 = fetch_param(mem, p1);
            let p2 = match p2 {
                Param::Position(n) => n,
                _ => panic!("Invalid parameter"),
            };
            mem[p2] = p0 * p1;
            *ip += 4;
        }
        Instr::Store(p0) => {
            let p0 = match p0 {
                Param::Position(n) => n,
                _ => panic!("Invalid parameter"),
            };
            let input = readnum();
            mem[p0] = input;
            *ip += 2;
        }
        Instr::Show(p0) => {
            let p0 = fetch_param(mem, p0);
            println!("{}", p0);
            *ip += 2;
        }
        Instr::JmpT(p0, p1) => {
            let p0 = fetch_param(mem, p0);
            if p0 == 0 {
                *ip += 3;
            } else {
                let p1 = fetch_param(mem, p1);
                *ip = p1.try_into().unwrap();
            }
        }
        Instr::JmpF(p0, p1) => {
            let p0 = fetch_param(mem, p0);
            if p0 == 0 {
                let p1 = fetch_param(mem, p1);
                *ip = p1.try_into().unwrap();
            } else {
                *ip += 3;
            }
        }
        Instr::CmpLt(p0, p1, p2) => {
            let p0 = fetch_param(mem, p0);
            let p1 = fetch_param(mem, p1);
            let p2 = match p2 {
                Param::Position(n) => n,
                _ => panic!("Invalid parameter"),
            };
            if p0 < p1 {
                mem[p2] = 1;
            } else {
                mem[p2] = 0;
            }
            *ip += 4;
        }
        Instr::CmpEq(p0, p1, p2) => {
            let p0 = fetch_param(mem, p0);
            let p1 = fetch_param(mem, p1);
            let p2 = match p2 {
                Param::Position(n) => n,
                _ => panic!("Invalid parameter"),
            };
            if p0 == p1 {
                mem[p2] = 1;
            } else {
                mem[p2] = 0;
            }
            *ip += 4;
        }
    }
    return true;
}

fn run_computer(ip: usize, mem: Vec<i32>) -> Vec<i32> {
    let mut ip = ip;
    let mut mem = mem;
    loop {
        if !run_instr(&mut ip, &mut mem) {
            break;
        }
    }
    return mem;
}

fn main() {
    println!("--- Day 5: Sunny with a Chance of Asteroids ---\n");

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

    println!("Please enter the ID 1, if asked for a value.\n");
    run_computer(0, memory.clone());

    println!("\n--- Part 2: ---\n");

    println!("Please enter the ID 5, if asked for a value.\n");
    run_computer(0, memory.clone());
}

#[test]
fn test_p1() {
    let input = "1002,4,3,4,33";
    let mem = parse_input(&input);

    let finalmem = run_computer(0, mem);

    assert_eq!(vec![1002, 4, 3, 4, 99], finalmem);
}

#[test]
fn test_p2() {
    // let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    // let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let mem = parse_input(&input);

    let finalmem = run_computer(0, mem);
}
