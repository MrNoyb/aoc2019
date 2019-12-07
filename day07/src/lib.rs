pub mod icm {
    use std::convert::TryInto;
    use std::io::{self, BufRead, Write};

    #[derive(Debug)]
    enum Param {
        Position(usize),
        Immediate(i32),
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

    pub struct Processor<'a, T: BufRead, U: Write>
    where
        T: 'a,
        U: 'a,
    {
        ip: usize,
        mem: Vec<i32>,
        input: &'a mut T,
        output: &'a mut U,
    }

    impl<'a, T: BufRead, U: Write> Processor<'a, T, U> {
        pub fn new(
            ip: usize,
            mem: Vec<i32>,
            input: &'a mut T,
            output: &'a mut U,
        ) -> Processor<'a, T, U> {
            Processor {
                ip,
                mem,
                input,
                output,
            }
        }

        pub fn set_ip(&mut self, ip: usize) {
            self.ip = ip;
        }

        pub fn set_memory(&mut self, mem: Vec<i32>) {
            self.mem = mem;
        }

        pub fn set_input(&mut self, input: &'a mut T) {
            self.input = input;
        }

        pub fn set_output(&mut self, output: &'a mut U) {
            self.output = output;
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
            // println!("INSTR: {:?}", i);
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
                    let p2 = match p2 {
                        Param::Position(n) => n,
                        _ => panic!("Invalid parameter"),
                    };
                    self.mem[p2] = p0 + p1;
                    self.ip += 4;
                }
                Instr::Mul(p0, p1, p2) => {
                    let p0 = self.fetch_param(p0);
                    let p1 = self.fetch_param(p1);
                    let p2 = match p2 {
                        Param::Position(n) => n,
                        _ => panic!("Invalid parameter"),
                    };
                    self.mem[p2] = p0 * p1;
                    self.ip += 4;
                }
                Instr::Store(p0) => {
                    let p0 = match p0 {
                        Param::Position(n) => n,
                        _ => panic!("Invalid parameter"),
                    };
                    let input = self.readnum();
                    self.mem[p0] = input;
                    self.ip += 2;
                }
                Instr::Show(p0) => {
                    let p0 = self.fetch_param(p0);
                    self.output
                        .write(format!("{}\n", p0).as_bytes())
                        .expect("Processor output write error.");
                    self.output.flush().expect("Processor output flush error.");
                    // println!("{}", p0);
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
                    let p2 = match p2 {
                        Param::Position(n) => n,
                        _ => panic!("Invalid parameter"),
                    };
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
                    let p2 = match p2 {
                        Param::Position(n) => n,
                        _ => panic!("Invalid parameter"),
                    };
                    if p0 == p1 {
                        self.mem[p2] = 1;
                    } else {
                        self.mem[p2] = 0;
                    }
                    self.ip += 4;
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
                    99 => Instr::Halt,
                    _ => Instr::Unknown,
                }
            } else {
                panic!("Invalid memory address");
            }
        }

        fn fetch_param(&self, p: Param) -> i32 {
            match p {
                Param::Position(n) => self.mem[n],
                Param::Immediate(n) => n,
            }
        }

        fn readnum(&mut self) -> i32 {
            // print!("Enter value: ");
            io::stdout().flush().expect("Could not flush stdout.");
            let mut input = String::new();
            match self.input.read_line(&mut input) {
                Ok(_) => input.trim().parse::<i32>().unwrap(),
                Err(_) => panic!("Could not read from stdin."),
            }
        }
    } // END IMPL Processor
}
