use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::Read;

#[test]
fn test_manhattan() {
    assert_eq!(7, manhattan((0, 0), (3, 4)));
    assert_eq!(7, manhattan((0, 0), (-4, -3)));
    assert_eq!(7, manhattan((0, 0), (4, 3)));
}

#[test]
fn test_p1t1() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let input = parse_input(&input);

    let origin = (0, 0);

    let path1 = dirs2points((0, 0), &input[0]);
    let path2 = dirs2points((0, 0), &input[1]);

    let hpath1: HashSet<_> = path1.iter().copied().collect();
    let hpath2: HashSet<_> = path2.iter().copied().collect();

    let inter: HashSet<_> = hpath1.intersection(&hpath2).copied().collect();
    let dists: Vec<i32> = inter.iter().map(|p| manhattan(origin, *p)).collect();

    let minimum = minimum(&dists);

    assert_eq!(159, minimum);
}

#[test]
fn test_p2t1() {
    let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let input = parse_input(&input);

    let path1 = dirs2points((0, 0), &input[0]);
    let path2 = dirs2points((0, 0), &input[1]);

    let hpath1: HashSet<_> = path1.iter().copied().collect();
    let hpath2: HashSet<_> = path2.iter().copied().collect();

    let inter: HashSet<_> = hpath1.intersection(&hpath2).copied().collect();

    let mut path1_dists = vec![];
    let mut path2_dists = vec![];

    for p in &inter {
        path1_dists.push(path1.iter().position(|x| *x == *p).unwrap());
        path2_dists.push(path2.iter().position(|x| *x == *p).unwrap());
    }

    let mut min_dists = vec![];
    for (x, y) in path1_dists.iter().zip(path2_dists.iter()) {
        min_dists.push(x + y + 2);
    }

    let min = minimum(&min_dists);

    assert_eq!(30, min);
}

#[test]
fn test_p2t2() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let input = parse_input(&input);

    let path1 = dirs2points((0, 0), &input[0]);
    let path2 = dirs2points((0, 0), &input[1]);

    let hpath1: HashSet<_> = path1.iter().copied().collect();
    let hpath2: HashSet<_> = path2.iter().copied().collect();

    let inter: HashSet<_> = hpath1.intersection(&hpath2).copied().collect();

    let mut path1_dists = vec![];
    let mut path2_dists = vec![];

    for p in &inter {
        path1_dists.push(path1.iter().position(|x| *x == *p).unwrap());
        path2_dists.push(path2.iter().position(|x| *x == *p).unwrap());
    }

    let mut min_dists = vec![];
    for (x, y) in path1_dists.iter().zip(path2_dists.iter()) {
        min_dists.push(x + y + 2);
    }

    let min = minimum(&min_dists);

    assert_eq!(610, min);
}

type Point = (i32, i32);

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn parse_input(s: &str) -> Vec<Vec<Direction>> {
    let mut v = vec![];

    for line in s.lines() {
        let mut u = vec![];
        for s in line.split(',') {
            let raw = s.trim();
            match raw.chars().next() {
                Some('U') => {
                    let n = raw[1..].parse::<i32>().unwrap();
                    u.push(Direction::Up(n));
                }
                Some('D') => {
                    let n = raw[1..].parse::<i32>().unwrap();
                    u.push(Direction::Down(n));
                }
                Some('L') => {
                    let n = raw[1..].parse::<i32>().unwrap();
                    u.push(Direction::Left(n));
                }
                Some('R') => {
                    let n = raw[1..].parse::<i32>().unwrap();
                    u.push(Direction::Right(n));
                }
                _ => panic!("Invalid direction."),
            }
        }
        v.push(u);
    }
    return v;
}

fn dirs2points(orig: Point, dirs: &[Direction]) -> Vec<Point> {
    let mut v = Vec::new();
    let mut p = orig;

    for d in dirs {
        match d {
            Direction::Up(n) => {
                for _ in 0..*n {
                    p.1 += 1;
                    v.push(p);
                }
            }
            Direction::Down(n) => {
                for _ in 0..*n {
                    p.1 -= 1;
                    v.push(p);
                }
            }
            Direction::Left(n) => {
                for _ in 0..*n {
                    p.0 -= 1;
                    v.push(p);
                }
            }
            Direction::Right(n) => {
                for _ in 0..*n {
                    p.0 += 1;
                    v.push(p);
                }
            }
        }
    }

    return v;
}

fn manhattan(p1: Point, p2: Point) -> i32 {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()).abs()
}

fn minimum<T: std::cmp::Ord + std::marker::Copy>(v: &[T]) -> T {
    v.iter()
        .copied()
        .fold(None, |min, x| match min {
            None => Some(x),
            Some(n) => Some(std::cmp::min(x, n)),
        })
        .unwrap()
}

fn main() {
    println!("--- Day 3: 1202 Program Alarm ---\n");

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

    let origin = (0, 0);

    let path1 = dirs2points(origin, &input[0]);
    let path2 = dirs2points(origin, &input[1]);

    let hpath1: HashSet<_> = path1.iter().copied().collect();
    let hpath2: HashSet<_> = path2.iter().copied().collect();

    let inter: HashSet<_> = hpath1.intersection(&hpath2).copied().collect();
    let dists: Vec<i32> = inter.iter().map(|p| manhattan(origin, *p)).collect();

    let min = minimum(&dists);

    println!("Minimum Distance: {}", min);

    println!("\n--- Part 2: ---\n");

    let mut path1_dists = vec![];
    let mut path2_dists = vec![];

    for p in &inter {
        path1_dists.push(path1.iter().position(|x| *x == *p).unwrap());
        path2_dists.push(path2.iter().position(|x| *x == *p).unwrap());
    }

    let mut min_dists = vec![];
    for (x, y) in path1_dists.iter().zip(path2_dists.iter()) {
        min_dists.push(x + y + 2);
    }

    let min = minimum(&min_dists);

    println!("Minimum Distance: {}", min);
}
