use num::Integer;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    pos: Vec<i64>,
    vel: Vec<i64>,
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            pos: vec![x, y, z],
            vel: vec![0, 0, 0],
        }
    }

    fn x_pos(&self) -> i64 {
        self.pos[0]
    }

    fn y_pos(&self) -> i64 {
        self.pos[1]
    }

    fn z_pos(&self) -> i64 {
        self.pos[2]
    }

    fn velocity(&self) -> Vec<i64> {
        self.vel.clone()
    }

    fn add_vec(v1: &[i64], v2: &[i64]) -> Vec<i64> {
        vec![v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
    }

    fn calc_gravity(m1: &Moon, m2: &Moon) -> Vec<i64> {
        let xs = m1.x_pos();
        let xo = m2.x_pos();
        let x_grav = if xs == xo {
            0
        } else if xs < xo {
            1
        } else {
            -1
        };
        let ys = m1.y_pos();
        let yo = m2.y_pos();
        let y_grav = if ys == yo {
            0
        } else if ys < yo {
            1
        } else {
            -1
        };
        let zs = m1.z_pos();
        let zo = m2.z_pos();
        let z_grav = if zs == zo {
            0
        } else if zs < zo {
            1
        } else {
            -1
        };
        vec![x_grav, y_grav, z_grav]
    }

    fn apply_gravity(&mut self, gv: &[Vec<i64>]) {
        gv.iter().for_each(|gravity| {
            self.vel = Moon::add_vec(&self.vel, &gravity);
        });
        //self.vel.add(&gravity));
    }

    fn apply_velocity(&mut self) {
        self.pos = Moon::add_vec(&self.pos, &self.vel);
    }

    fn potential_energy(&self) -> i64 {
        self.pos.iter().map(|x| x.abs()).sum()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vel.iter().map(|x| x.abs()).sum()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn step_time(n: usize, moons: &mut [Moon]) {
    let mut cpy = moons.to_vec();
    (0..n).for_each(|_| {
        for moon in moons.iter_mut() {
            let tmp = moon.clone();
            let mut gv = vec![];
            for other in cpy.iter().filter(|m| **m != tmp) {
                gv.push(Moon::calc_gravity(moon, other));
            }
            moon.apply_gravity(&gv[..]);
            moon.apply_velocity();
        }
        cpy = moons.to_vec();
    });
}

fn parse_input(s: &str) -> Vec<Moon> {
    let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    let mut v = vec![];
    for line in s.lines() {
        if let Some(caps) = re.captures(line) {
            let x = caps
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .expect("Could not parse number");
            let y = caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .expect("Could not parse number");
            let z = caps
                .get(3)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .expect("Could not parse number");
            v.push(Moon::new(x, y, z));
        } else {
            panic!("Parsing of input failed by {}", line);
        }
    }
    return v;
}

fn get_all_axis(moons: &[Moon]) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let x_vec = moons.iter().map(|m| m.x_pos()).collect::<Vec<_>>();
    let y_vec = moons.iter().map(|m| m.y_pos()).collect::<Vec<_>>();
    let z_vec = moons.iter().map(|m| m.z_pos()).collect::<Vec<_>>();
    (x_vec, y_vec, z_vec)
}

fn get_all_velocities(moons: &[Moon]) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let v: Vec<Vec<i64>> = moons.iter().map(|m| m.velocity()).collect();
    let x_vec = v.iter().map(|m| m[0]).collect::<Vec<_>>();
    let y_vec = v.iter().map(|m| m[1]).collect::<Vec<_>>();
    let z_vec = v.iter().map(|m| m[2]).collect::<Vec<_>>();
    (x_vec, y_vec, z_vec)
}

fn main() {
    println!("--- Day 12: The N-Body Problem ---\n");

    println!("Reading input...");

    let input = include_str!("../input");

    println!("Parsing input...");
    let moons = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    let mut system = moons.clone();

    step_time(1000, &mut system);

    let total_energy: i64 = system.iter().map(|m| m.total_energy()).sum();

    println!("Total energy after 100 steps: {}", total_energy);

    println!("\n--- Part 2: ---\n");

    let mut system = moons.clone();

    let mut steps_vec = vec![];
    let mut steps: u64 = 0;

    let mut history_x: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();
    let mut history_y: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();
    let mut history_z: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();

    let mut search_x = true;
    let mut search_y = true;
    let mut search_z = true;

    while steps_vec.len() < 3 {
        let (x_pos, y_pos, z_pos) = get_all_axis(&system);
        let (x_vel, y_vel, z_vel) = get_all_velocities(&system);
        if search_x {
            if !history_x.insert((x_pos, x_vel)) {
                steps_vec.push(steps);
                search_x = false;
            }
        }
        if search_y {
            if !history_y.insert((y_pos, y_vel)) {
                steps_vec.push(steps);
                search_y = false;
            }
        }
        if search_z {
            if !history_z.insert((z_pos, z_vel)) {
                steps_vec.push(steps);
                search_z = false;
            }
        }
        steps += 1;
        step_time(1, &mut system);
    }

    let res = steps_vec[0].lcm(&steps_vec[1]).lcm(&steps_vec[2]);

    println!("History repeats after {} steps.", res);
}

#[test]
fn test_p1t1() {
    let mut moons = vec![
        Moon::new(-1, 0, 2),
        Moon::new(2, -10, -7),
        Moon::new(4, -8, 8),
        Moon::new(3, 5, -1),
    ];
    let moons_0 = moons.clone();
    let moons_1 = vec![
        Moon {
            pos: vec![2, -1, 1],
            vel: vec![3, -1, -1],
        },
        Moon {
            pos: vec![3, -7, -4],
            vel: vec![1, 3, 3],
        },
        Moon {
            pos: vec![1, -7, 5],
            vel: vec![-3, 1, -3],
        },
        Moon {
            pos: vec![2, 2, 0],
            vel: vec![-1, -3, 1],
        },
    ];
    let moons_2 = vec![
        Moon {
            pos: vec![5, -3, -1],
            vel: vec![3, -2, -2],
        },
        Moon {
            pos: vec![1, -2, 2],
            vel: vec![-2, 5, 6],
        },
        Moon {
            pos: vec![1, -4, -1],
            vel: vec![0, 3, -6],
        },
        Moon {
            pos: vec![1, -4, 2],
            vel: vec![-1, -6, 2],
        },
    ];
    let moons_3 = vec![
        Moon {
            pos: vec![5, -6, -1],
            vel: vec![0, -3, 0],
        },
        Moon {
            pos: vec![0, 0, 6],
            vel: vec![-1, 2, 4],
        },
        Moon {
            pos: vec![2, 1, -5],
            vel: vec![1, 5, -4],
        },
        Moon {
            pos: vec![1, -8, 2],
            vel: vec![0, -4, 0],
        },
    ];
    let moons_4 = vec![
        Moon {
            pos: vec![2, -8, 0],
            vel: vec![-3, -2, 1],
        },
        Moon {
            pos: vec![2, 1, 7],
            vel: vec![2, 1, 1],
        },
        Moon {
            pos: vec![2, 3, -6],
            vel: vec![0, 2, -1],
        },
        Moon {
            pos: vec![2, -9, 1],
            vel: vec![1, -1, -1],
        },
    ];
    let moons_5 = vec![
        Moon {
            pos: vec![-1, -9, 2],
            vel: vec![-3, -1, 2],
        },
        Moon {
            pos: vec![4, 1, 5],
            vel: vec![2, 0, -2],
        },
        Moon {
            pos: vec![2, 2, -4],
            vel: vec![0, -1, 2],
        },
        Moon {
            pos: vec![3, -7, -1],
            vel: vec![1, 2, -2],
        },
    ];
    let moons_10 = vec![
        Moon {
            pos: vec![2, 1, -3],
            vel: vec![-3, -2, 1],
        },
        Moon {
            pos: vec![1, -8, 0],
            vel: vec![-1, 1, 3],
        },
        Moon {
            pos: vec![3, -6, 1],
            vel: vec![3, 2, -3],
        },
        Moon {
            pos: vec![2, 0, 4],
            vel: vec![1, -1, -1],
        },
    ];

    step_time(0, &mut moons);
    assert_eq!(moons, moons_0);

    step_time(1, &mut moons);
    assert_eq!(moons, moons_1);

    step_time(1, &mut moons);
    assert_eq!(moons, moons_2);

    step_time(1, &mut moons);
    assert_eq!(moons, moons_3);

    step_time(1, &mut moons);
    assert_eq!(moons, moons_4);

    step_time(1, &mut moons);
    assert_eq!(moons, moons_5);

    step_time(5, &mut moons);
    assert_eq!(moons, moons_10);

    let total_energy_10 = 179;
    let total_energy: i64 = moons.iter().map(|m| m.total_energy()).sum();

    assert_eq!(total_energy, total_energy_10);
}

#[test]
fn test_p2t1() {
    let moons = vec![
        Moon::new(-1, 0, 2),
        Moon::new(2, -10, -7),
        Moon::new(4, -8, 8),
        Moon::new(3, 5, -1),
    ];
    let mut system = moons.clone();

    let mut steps_vec = vec![];
    let mut steps = 0;

    let mut history_x: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();
    let mut history_y: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();
    let mut history_z: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();

    let mut search_x = true;
    let mut search_y = true;
    let mut search_z = true;

    while steps_vec.len() < 3 {
        let (x_pos, y_pos, z_pos) = get_all_axis(&system);
        let (x_vel, y_vel, z_vel) = get_all_velocities(&system);
        if search_x {
            if !history_x.insert((x_pos, x_vel)) {
                steps_vec.push(steps);
                search_x = false;
            }
        }
        if search_y {
            if !history_y.insert((y_pos, y_vel)) {
                steps_vec.push(steps);
                search_y = false;
            }
        }
        if search_z {
            if !history_z.insert((z_pos, z_vel)) {
                steps_vec.push(steps);
                search_z = false;
            }
        }
        steps += 1;
        step_time(1, &mut system);
    }

    println!("Steps: {:?}", steps_vec);
    let res = steps_vec[0].lcm(&steps_vec[1]).lcm(&steps_vec[2]);

    assert_eq!(res, 2772);
}

#[test]
fn test_p2t2() {
    let moons =
        parse_input("<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>");

    let mut system = moons.clone();

    let mut steps_vec = vec![];
    let mut steps: u64 = 0;

    let mut history_x: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();
    let mut history_y: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();
    let mut history_z: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();

    let mut search_x = true;
    let mut search_y = true;
    let mut search_z = true;

    while steps_vec.len() < 3 {
        let (x_pos, y_pos, z_pos) = get_all_axis(&system);
        let (x_vel, y_vel, z_vel) = get_all_velocities(&system);
        if search_x {
            if !history_x.insert((x_pos, x_vel)) {
                steps_vec.push(steps);
                search_x = false;
            }
        }
        if search_y {
            if !history_y.insert((y_pos, y_vel)) {
                steps_vec.push(steps);
                search_y = false;
            }
        }
        if search_z {
            if !history_z.insert((z_pos, z_vel)) {
                steps_vec.push(steps);
                search_z = false;
            }
        }
        steps += 1;
        step_time(1, &mut system);
    }

    println!("Steps: {:?}", steps_vec);
    let res = steps_vec[0].lcm(&steps_vec[1]).lcm(&steps_vec[2]);

    assert_eq!(res, 4686774924);
}
