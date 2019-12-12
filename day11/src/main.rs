use day11::icm::Processor;
use drawille::Canvas;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::Read;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

struct HullBot {
    x: i32,
    y: i32,
    dir: (i32, i32),
    painted: Vec<(i32, i32)>,
    panels: HashMap<(i32, i32), i128>,
    input: Receiver<i128>,
    output: Sender<i128>,
}

impl HullBot {
    fn new(inp: Receiver<i128>, out: Sender<i128>) -> HullBot {
        HullBot {
            x: 0,
            y: 0,
            dir: (0, 1),
            painted: vec![],
            panels: HashMap::new(),
            input: inp,
            output: out,
        }
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.dir = (0, 1);
        self.painted.clear();
        self.panels.clear();
    }

    fn paint_white(&mut self) {
        self.panels.insert((self.x, self.y), 1);
    }

    fn run(&mut self) {
        let timeout = Duration::from_millis(100);
        loop {
            if let Some(color) = self.panels.get(&(self.x, self.y)) {
                self.output.send(*color).expect("Sending to cpu failed.");
            } else {
                self.panels.insert((self.x, self.y), 0);
                self.output.send(0).expect("Sending to cpu failed.");
            }
            let paint_color = match self.input.recv_timeout(timeout) {
                Ok(c) => c,
                _ => break,
            };
            self.panels.insert((self.x, self.y), paint_color);
            if paint_color == 1 {
                self.painted.push((self.x, self.y));
            }
            let turn_dir = self.input.recv().unwrap();
            if turn_dir == 0 {
                self.turn_left();
            } else {
                self.turn_right();
            }
            self.forward();
        }
    }

    fn turn_left(&mut self) {
        match self.dir {
            (0, 1) => self.dir = (-1, 0),
            (-1, 0) => self.dir = (0, -1),
            (0, -1) => self.dir = (1, 0),
            (1, 0) => self.dir = (0, 1),
            _ => panic!("Not a valid direction!"),
        };
    }

    fn turn_right(&mut self) {
        match self.dir {
            (0, 1) => self.dir = (1, 0),
            (1, 0) => self.dir = (0, -1),
            (0, -1) => self.dir = (-1, 0),
            (-1, 0) => self.dir = (0, 1),
            _ => panic!("Not a valid direction!"),
        };
    }

    fn forward(&mut self) {
        self.x += self.dir.0;
        self.y += self.dir.1;
    }
}

fn main() {
    println!("--- Day 11: Space Police ---\n");

    println!("Reading input...");

    let mut input = String::new();
    {
        let mut file = File::open("input").expect("Could not open input file.");
        file.read_to_string(&mut input)
            .expect("Could not read from input file.");
    }

    println!("Parsing input...");
    let program = parse_input(&input);

    let (send_cpu, recv_cpu) = channel();
    let (send_bot, recv_bot) = channel();

    println!("\n--- Part 1: ---\n");

    let cpu = Arc::new(Mutex::new(Processor::new(
        0,
        program.clone(),
        recv_cpu,
        send_bot,
    )));
    let bot = Arc::new(Mutex::new(HullBot::new(recv_bot, send_cpu)));

    let bot_handle = Arc::clone(&bot);
    let tbot = thread::spawn(move || {
        let mut bot = bot_handle.lock().unwrap();
        bot.run();
    });
    let cpu_handle = Arc::clone(&cpu);
    let tcpu = thread::spawn(move || {
        let mut cpu = cpu_handle.lock().unwrap();
        cpu.run();
    });

    tcpu.join().expect("CPU join error");
    tbot.join().expect("BOT join error");

    {
        let bot = bot.lock().unwrap();
        println!("visited panels: {}", bot.panels.len());
    }

    println!("\n--- Part 2: ---\n");

    {
        let mut bot = bot.lock().unwrap();
        bot.reset();
        bot.paint_white();
    }

    {
        let mut cpu = cpu.lock().unwrap();
        cpu.reset();
        cpu.load_into_memory(&program);
    }

    let bot_handle = Arc::clone(&bot);
    let tbot = thread::spawn(move || {
        let mut bot = bot_handle.lock().unwrap();
        bot.run();
    });
    let cpu_handle = Arc::clone(&cpu);
    let tcpu = thread::spawn(move || {
        let mut cpu = cpu_handle.lock().unwrap();
        cpu.run();
    });

    tcpu.join().expect("CPU join error");
    tbot.join().expect("BOT join error");

    {
        let bot = bot.lock().unwrap();
        let points = &bot.painted;
        let mut canvas = Canvas::new(50, 10);
        points
            .iter()
            .for_each(|p| canvas.set(p.0.abs().try_into().unwrap(), p.1.abs().try_into().unwrap()));

        println!("{}", canvas.frame());
    }
}
