pub mod icm {
    use std::convert::TryInto;
    use std::sync::mpsc::{Receiver, Sender};

    #[derive(Debug)]
    enum Param {
        Immediate(i32),
        Position(usize),
        Relative(i32),
    }

    /*
    Parses a parameter.
    nth   - Number of the parameter, beginning by zero.
    pcode - Paramter mode code. A number where each digit specifies a parameter mode.
    val   - Value of the parameter.
     */
    fn parse_param(nth: u32, pcode: i32, val: i32) -> Param {
        match (pcode / 10_i32.pow(nth)) % 10 {
            0 => {
                let p: usize = val.try_into().unwrap();
                Param::Position(p)
            }
            1 => Param::Immediate(val),
            2 => Param::Relative(val),
            _ => panic!("Invalide pcode."),
        }
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
        RBase(Param),
    }

    pub struct Processor {
        ip: usize,
        rel_base: i32,
        mem: Vec<i32>,
        input: Receiver<i32>,
        output: Sender<i32>,
    }

    impl Processor {
        pub fn new(
            ip: usize,
            mem: Vec<i32>,
            input: Receiver<i32>,
            output: Sender<i32>,
        ) -> Processor {
            Processor {
                ip,
                rel_base: 0,
                mem,
                input,
                output,
            }
        }

        pub fn reset(&mut self) {
            self.ip = 0;
            self.rel_base = 0;
        }

        pub fn memory_from_str(&mut self, s: &str) {
            let mut v = vec![];
            for n in s.split(',') {
                let n = match n.trim().parse::<i32>() {
                    Err(e) => panic!("Could not parse {}: {}", n, e),
                    Ok(i) => i,
                };
                v.push(n);
            }
            self.load_into_memory(&v);
        }

        pub fn load_into_memory(&mut self, mem: &[i32]) {
            if mem.len() > self.mem.len() {
                self.mem.resize(mem.len(), 0);
            }
            self.mem.clear();
            self.mem.extend_from_slice(mem);
        }

        pub fn set_ip(&mut self, ip: usize) {
            self.ip = ip;
        }

        pub fn set_memory(&mut self, mem: Vec<i32>) {
            self.mem = mem;
        }

        pub fn set_address(&mut self, ind: usize, val: i32) {
            self.safecheck_memory(ind);
            self.mem[ind] = val;
        }

        pub fn set_input(&mut self, input: Receiver<i32>) {
            self.input = input;
        }

        pub fn get_input(&self) -> &Receiver<i32> {
            return &self.input;
        }

        pub fn set_output(&mut self, output: Sender<i32>) {
            self.output = output;
        }

        fn safecheck_memory(&mut self, ind: usize) {
            if ind >= self.mem.len() {
                self.mem.resize(ind * 2, 0);
            }
        }

        pub fn run(&mut self) {
            loop {
                if !self.run_instr() {
                    break;
                }
            }
        }

        fn run_instr(&mut self) -> bool {
            let i = self.fetch_instruction();
            // println!("{:>08}: {:?}", self.ip, i);
            match i {
                Instr::Unknown => {
                    println!("PANIC: Invalid instruction");
                    return false;
                }
                Instr::Halt => {
                    // println!("HALT");
                    return false;
                }
                Instr::Add(p0, p1, p2) => {
                    let p0 = self.fetch_param(p0);
                    let p1 = self.fetch_param(p1);
                    let p2 = self.fetch_addr(p2);
                    self.mem[p2] = p0 + p1;
                    self.ip += 4;
                }
                Instr::Mul(p0, p1, p2) => {
                    let p0 = self.fetch_param(p0);
                    let p1 = self.fetch_param(p1);
                    let p2 = self.fetch_addr(p2);
                    self.mem[p2] = p0 * p1;
                    self.ip += 4;
                }
                Instr::Store(p0) => {
                    let p0 = self.fetch_addr(p0);
                    if let Ok(input) = self.input.recv() {
                        self.mem[p0] = input;
                        self.ip += 2;
                    } else {
                        // println!("Processor input channel receive error. HALTING");
                        return false;
                    }
                }
                Instr::Show(p0) => {
                    let p0 = self.fetch_param(p0);
                    self.output
                        .send(p0)
                        .expect("Processor output channel send error.");
                    self.ip += 2;
                }
                Instr::JmpT(p0, p1) => {
                    let p0 = self.fetch_param(p0);
                    if p0 == 0 {
                        self.ip += 3;
                    } else {
                        let p1 = self.fetch_param(p1);
                        self.ip = p1.try_into().unwrap();
                    }
                }
                Instr::JmpF(p0, p1) => {
                    let p0 = self.fetch_param(p0);
                    if p0 == 0 {
                        let p1 = self.fetch_param(p1);
                        self.ip = p1.try_into().unwrap();
                    } else {
                        self.ip += 3;
                    }
                }
                Instr::CmpLt(p0, p1, p2) => {
                    let p0 = self.fetch_param(p0);
                    let p1 = self.fetch_param(p1);
                    let p2 = self.fetch_addr(p2);
                    if p0 < p1 {
                        self.mem[p2] = 1;
                    } else {
                        self.mem[p2] = 0;
                    }
                    self.ip += 4;
                }
                Instr::CmpEq(p0, p1, p2) => {
                    let p0 = self.fetch_param(p0);
                    let p1 = self.fetch_param(p1);
                    let p2 = self.fetch_addr(p2);
                    if p0 == p1 {
                        self.mem[p2] = 1;
                    } else {
                        self.mem[p2] = 0;
                    }
                    self.ip += 4;
                }
                Instr::RBase(p0) => {
                    let p0 = self.fetch_param(p0);
                    self.rel_base += p0;
                    self.ip += 2;
                }
            }
            return true;
        }

        fn fetch_instruction(&self) -> Instr {
            if let Some(val) = self.mem.get(self.ip) {
                let opcode = val % 100;
                let pcode = val / 100;

                match opcode {
                    1 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        let p1 = parse_param(1, pcode, self.mem[self.ip + 2]);
                        let p2 = parse_param(2, pcode, self.mem[self.ip + 3]);
                        Instr::Add(p0, p1, p2)
                    }
                    2 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        let p1 = parse_param(1, pcode, self.mem[self.ip + 2]);
                        let p2 = parse_param(2, pcode, self.mem[self.ip + 3]);
                        Instr::Mul(p0, p1, p2)
                    }
                    3 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        Instr::Store(p0)
                    }
                    4 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        Instr::Show(p0)
                    }
                    5 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        let p1 = parse_param(1, pcode, self.mem[self.ip + 2]);
                        Instr::JmpT(p0, p1)
                    }
                    6 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        let p1 = parse_param(1, pcode, self.mem[self.ip + 2]);
                        Instr::JmpF(p0, p1)
                    }
                    7 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        let p1 = parse_param(1, pcode, self.mem[self.ip + 2]);
                        let p2 = parse_param(2, pcode, self.mem[self.ip + 3]);
                        Instr::CmpLt(p0, p1, p2)
                    }
                    8 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        let p1 = parse_param(1, pcode, self.mem[self.ip + 2]);
                        let p2 = parse_param(2, pcode, self.mem[self.ip + 3]);
                        Instr::CmpEq(p0, p1, p2)
                    }
                    9 => {
                        let p0 = parse_param(0, pcode, self.mem[self.ip + 1]);
                        Instr::RBase(p0)
                    }
                    99 => Instr::Halt,
                    _ => Instr::Unknown,
                }
            } else {
                panic!("Invalid memory address");
            }
        }

        fn fetch_param(&mut self, p: Param) -> i32 {
            match p {
                Param::Immediate(n) => n,
                Param::Position(n) => {
                    self.safecheck_memory(n);
                    self.mem[n]
                }
                Param::Relative(n) => {
                    let ind = (self.rel_base + n) as usize;
                    self.safecheck_memory(ind);
                    self.mem[ind]
                }
            }
        }

        fn fetch_addr(&mut self, p: Param) -> usize {
            let addr: usize = match p {
                Param::Position(n) => n.try_into().unwrap(),
                Param::Relative(n) => (self.rel_base + n).try_into().unwrap(),
                _ => panic!("Invalid paramater!"),
            };
            self.safecheck_memory(addr);
            return addr;
        }
    } // END IMPL Processor
}
