use day15::icm::Processor;
use itertools::Itertools;
use itertools::MinMaxResult;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type Coord = (i32, i32);
type Map = HashMap<Coord, i32>;

struct Tracker {
    pos: Coord,
    visited: HashSet<Coord>,
    map: Map,
    input: Receiver<i32>,
    output: Sender<i32>,
}

impl Tracker {
    fn new(input: Receiver<i32>, output: Sender<i32>) -> Self {
        let mut init = HashMap::new();
        init.insert((0, 0), 1);
        Tracker {
            pos: (0, 0),
            visited: HashSet::new(),
            map: init,
            input,
            output,
        }
    }

    fn get_oxygen_system(&self) -> Option<Coord> {
        let sys = self
            .map
            .iter()
            .filter(|(_, v)| **v == 2)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();
        if let Some(c) = sys.get(0) {
            Some(**c)
        } else {
            None
        }
    }

    fn get_paths(&self) -> Vec<Coord> {
        self.map
            .iter()
            .filter(|(_, v)| **v > 0)
            .map(|(k, _)| k)
            .copied()
            .collect()
    }

    fn neighbors(&self) -> Vec<(Coord, i32)> {
        let (x, y) = self.pos;
        vec![
            ((x, y + 1), 1),
            ((x, y - 1), 2),
            ((x - 1, y), 3),
            ((x + 1, y), 4),
        ]
        .iter()
        .copied()
        .filter(|c| !self.visited.contains(&c.0))
        .collect()
    }

    fn mv(&self, dir: i32) -> i32 {
        // println!("Moving in {}", dir);
        self.output.send(dir).expect("Could not send dir to cpu");
        self.input.recv().expect("Could not receive from cpu")
    }

    fn map(&mut self) {
        // use flood fill to create a map
        let mut todo = vec![(self.pos, 0)];
        self.visited.insert(self.pos);

        while let Some((coord, dir)) = todo.pop() {
            if dir != 0 {
                let stat = self.mv(dir);
                self.map.insert(coord, stat);
                if stat == 1 {
                    // safe current position
                    let prev_pos = self.pos;
                    self.pos = coord;
                    // map from new position
                    self.map();
                    // go back to previous position
                    self.pos = prev_pos;
                    self.mv(Tracker::rev_dir(dir));
                } else if stat == 2 {
                    self.visited.insert(coord);
                    self.mv(Tracker::rev_dir(dir));
                } else {
                    self.visited.insert(coord);
                }
            }

            for (neigh, dir) in self.neighbors() {
                todo.push((neigh, dir));
            }
            // self.print_map();
        }
    }

    // Returns (min, max) Coordinates of the map.
    fn get_map_boundaries(&self) -> Option<(Coord, Coord)> {
        let minmax_x = self.map.keys().map(|(x, _)| x).minmax();
        let minmax_y = self.map.keys().map(|(_, y)| y).minmax();
        match (minmax_x, minmax_y) {
            (MinMaxResult::MinMax(xb, xg), MinMaxResult::MinMax(yb, yg)) => {
                Some(((*xb, *yb), (*xg, *yg)))
            }
            _ => None,
        }
    }

    fn print_map(&self) {
        let mut s = String::new();
        if let Some(((xb, yb), (xg, yg))) = self.get_map_boundaries() {
            for y in yb..=yg {
                for x in xb..=xg {
                    if (x, y) == self.pos {
                        s.push('X');
                        continue;
                    }
                    if let Some(n) = self.map.get(&(x, y)) {
                        match n {
                            0 => s.push('\u{2588}'),
                            1 => s.push(' '),
                            2 => s.push('O'),
                            _ => s.push('!'),
                        }
                    } else {
                        s.push('.');
                    }
                }
                s.push('\n');
            }
            // print!("\x1B[2J");
            print!("{}", s);
        } else {
            println!("No map available.");
        }
    }

    fn rev_dir(dir: i32) -> i32 {
        match dir {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => panic!("Unknown direction: {:?}", dir),
        }
    }
}

fn neighbors(c: &Coord) -> Vec<Coord> {
    let (x, y) = *c;
    vec![(x, y + 1), (x, y - 1), (x - 1, y), (x + 1, y)]
}

fn path_lengths(start: &Coord, paths: &[Coord]) -> HashMap<Coord, u32> {
    let mut map = HashMap::new();

    let mut todo = vec![(*start, 0)];
    while let Some((cur, plen)) = todo.pop() {
        neighbors(&cur).iter().for_each(|c| {
            if paths.contains(c) && !map.contains_key(c) {
                map.insert(*c, plen + 1);
                todo.push((*c, plen + 1));
            }
        })
    }

    map
}

fn main() {
    println!("--- Day 15: Oxygen System ---\n");

    println!("Reading input...");

    let input = include_str!("../input");

    let (send_cpu, cpu_in) = channel();
    let (cpu_out, recv_cpu) = channel();

    let cpu = Arc::new(Mutex::new(Processor::new(0, vec![], cpu_in, cpu_out)));
    {
        let mut cpu = cpu.lock().unwrap();
        cpu.memory_from_str(&input);
    }

    println!("\n--- Part 1: ---\n");

    let cpu_handle = Arc::clone(&cpu);
    thread::spawn(move || {
        let mut cpu = cpu_handle.lock().unwrap();
        cpu.run();
    });

    let mut tracker = Tracker::new(recv_cpu, send_cpu);

    // map the station
    tracker.map();

    println!("Ship layout:\n");
    tracker.print_map();
    println!("\nRepair Bot (X) is at {:?}", tracker.pos);
    let oxysys = tracker.get_oxygen_system().unwrap();
    println!("Oxygen System (O) is at {:?}", oxysys);
    let paths = tracker.get_paths();
    let plen = path_lengths(&(0, 0), &paths);
    println!(
        "\nOxygen System is {} commands away.",
        plen.get(&oxysys).unwrap()
    );

    println!("\n--- Part 2: ---\n");

    let oxy_paths = path_lengths(&oxysys, &paths);
    // get the coord which is the farthest from the oxygen sytem
    if let Some(v) = oxy_paths.iter().map(|(_, v)| v).max() {
        println!("It takes {} minutes to fill everything with oxygen.", v);
    } else {
        println!("Could not determine farthest point.");
    }
}
