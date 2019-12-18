use day13::icm::Processor;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl Tile {
    fn from_id(id: i32) -> Tile {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPaddle,
            4 => Tile::Ball,
            _ => panic!("Tile parsing error: Unknown Tile ID."),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Arcade {
    tiles: HashMap<Point, Tile>,
    joystick_pos: i32,
    display: i32,
    inp: Receiver<i32>,
    out: Sender<i32>,
}

impl Arcade {
    fn new(inp: Receiver<i32>, out: Sender<i32>) -> Arcade {
        Arcade {
            tiles: HashMap::new(),
            joystick_pos: 0,
            display: 0,
            inp,
            out,
        }
    }

    fn reset(&mut self) {
        self.tiles.clear();
        self.joystick_pos = 0;
        self.display = 0;
    }

    fn tiles(&self) -> Values<Point, Tile> {
        self.tiles.values()
    }

    fn display(&self) -> i32 {
        self.display
    }

    fn get_ball_pos(&self) -> Option<Point> {
        self.tiles
            .iter()
            .filter(|(_k, v)| **v == Tile::Ball)
            .map(|(k, _)| *k)
            .next()
    }

    fn get_paddle_pos(&self) -> Option<Point> {
        self.tiles
            .iter()
            .filter(|(_k, v)| **v == Tile::HPaddle)
            .map(|(k, _)| *k)
            .next()
    }

    fn print_tiles(&self) -> String {
        let mut s = String::new();
        for j in 0..20 {
            for i in 0..50 {
                if let Some(tile) = self.tiles.get(&Point { x: i, y: j }) {
                    let c = match tile {
                        Tile::Empty => ' ',
                        Tile::Ball => '\u{2022}',
                        Tile::Block => '\u{2592}',
                        Tile::HPaddle => '\u{2015}',
                        Tile::Wall => '\u{2588}',
                    };
                    s.push(c);
                }
            }
            s.push('\n');
        }
        return s;
    }

    fn init(&mut self) {
        loop {
            if let Some((x, y, tid)) = self.read_three() {
                self.tiles.insert(Point::new(x, y), Tile::from_id(tid));
            } else {
                break;
            }
        }
    }

    fn play(&mut self) {
        let mut paddle = self.get_paddle_pos();
        let mut ball = self.get_ball_pos();
        let mut prev_ball = None;

        loop {
            if let Some(p) = paddle {
                if ball != prev_ball {
                    if let Some(b) = ball {
                        if p.x < b.x {
                            self.joystick_pos = 1;
                        } else if p.x > b.x {
                            self.joystick_pos = -1;
                        } else {
                            self.joystick_pos = 0;
                        }
                        self.out
                            .send(self.joystick_pos)
                            .expect("Could not send JS position");
                    } else {
                        self.joystick_pos = 0;
                    }
                    prev_ball = ball;
                    // print!("\x1B[2J");
                    // println!("{}", self.print_tiles());
                    // println!(
                    //     "Blocks: {}",
                    //     self.tiles().filter(|t| **t == Tile::Block).count()
                    // );
                    // println!("Score: {}", self.display());
                }
            }

            while let Some((x, y, tid)) = self.read_three() {
                if x == -1 && y == 0 {
                    self.display = tid;
                } else {
                    self.tiles.insert(Point::new(x, y), Tile::from_id(tid));
                }
            }

            if self.tiles().filter(|t| **t == Tile::Block).count() == 0 {
                break;
            }

            paddle = self.get_paddle_pos();
            ball = self.get_ball_pos();
        }
    }

    fn read_three(&self) -> Option<(i32, i32, i32)> {
        let timeout = Duration::from_millis(100);
        let one = match self.inp.recv_timeout(timeout) {
            Ok(v) => v,
            _ => return None,
        };
        let two = match self.inp.recv_timeout(timeout) {
            Ok(v) => v,
            _ => return None,
        };
        let three = match self.inp.recv_timeout(timeout) {
            Ok(v) => v,
            _ => return None,
        };

        Some((one, two, three))
    }
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

fn main() {
    println!("--- Day 13: Care Package ---\n");

    println!("Reading input...");

    let input = include_str!("../input");

    println!("Parsing input...");
    let program = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    let (send_cpu, recv_cpu) = channel();
    let (send_arcade, recv_arcade) = channel();
    let cpu = Arc::new(Mutex::new(Processor::new(
        0,
        program.clone(),
        recv_cpu,
        send_arcade,
    )));

    let mut arcade = Arcade::new(recv_arcade, send_cpu);

    let cpu_hdl = Arc::clone(&cpu);
    thread::spawn(move || {
        let mut cpu = cpu_hdl.lock().unwrap();
        cpu.run();
    });

    arcade.init();

    println!("{}", arcade.print_tiles());
    println!(
        "Number of Block Tiles: {}",
        arcade.tiles().filter(|t| **t == Tile::Block).count()
    );

    println!("\n--- Part 2: ---\n");

    {
        let mut cpu = cpu.lock().unwrap();
        cpu.reset();
        cpu.load_into_memory(&program);
        cpu.set_address(0, 2);
    }

    let cpu_hdl = Arc::clone(&cpu);
    thread::spawn(move || {
        let mut cpu = cpu_hdl.lock().unwrap();
        cpu.run();
    });

    arcade.reset();
    arcade.play();

    println!("Final Score: {}", arcade.display());
}
