use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Read;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, o: Point) -> f64 {
        let v = o - *self;
        let x = v.x as f64;
        let y = v.y as f64;
        return (x * x + y * y).sqrt();
    }

    fn magnitude(&self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;
        (x * x + y * y).sqrt()
    }

    fn angle(&self, o: Point) -> f64 {
        let v = o - *self;
        let x = v.x as f64;
        let y = v.y as f64;
        y.atan2(x)
    }

    fn scalar(&self, o: Point) -> f64 {
        (self.x * o.x + self.y * o.y) as f64
    }

    fn vec_angle(&self, o: Point) -> f64 {
        (self.scalar(o) / (self.magnitude() * o.magnitude())).acos()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn line_of_sight(pts: &Vec<Point>, p1: Point, p2: Point) -> bool {
    let angle = p1.angle(p2);
    let distance = p1.distance(p2);

    for p in pts {
        if *p == p1 || *p == p2 {
            continue;
        }
        let a = p1.angle(*p);
        let d = p1.distance(*p);
        // println!("for {}: {} == {}", p, angle, a);
        if a == angle && d < distance {
            return false;
        }
    }
    true
}

fn detect(pts: &Vec<Point>, p: Point) -> usize {
    let count = pts.iter().filter(|q| line_of_sight(pts, p, **q)).count();
    // println!("SCAN: {} -> {}", p, count);
    count - 1
}

fn best_station(pts: &Vec<Point>) -> (Point, usize) {
    pts.iter()
        .copied()
        .zip(pts.iter().map(|p| detect(pts, *p)))
        .max_by_key(|e| e.1)
        .unwrap()
}

fn vaporize(pts: &Vec<Point>, station: Point, n: usize) -> Point {
    let cur_dir = Point { x: 0, y: 1 };

    let angles = pts
        .iter()
        .copied()
        .map(|p| (p, cur_dir.vec_angle(p)))
        .collect::<Vec<_>>();

    let sorted = angles
        .iter()
        .min_by(|t1, t2| t1.1.cmp(t2.1).unwrap_or(Equal))
        .unwrap();

    println!("{:?}", angles);
    println!("{:?}", sorted);

    for _ in 0..n {}

    station
}

fn parse_input(s: &str) -> Vec<Point> {
    let mut v = vec![];

    for (y, l) in s.lines().enumerate() {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                v.push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        });
    }

    return v;
}

fn main() {
    println!("--- Day 10: Monitoring Station ---\n");

    println!("Reading input...");

    let mut input = String::new();
    {
        let mut file = File::open("input").expect("Could not open input file.");
        file.read_to_string(&mut input)
            .expect("Could not read from input file.");
    }

    println!("Parsing input...");
    let points = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    let best = best_station(&points);
    println!("Best station: {}", best.0);
    println!("Nr of detected: {}", best.1);

    println!("\n--- Part 2: ---\n");

    let station = best.0;
    // let asteroids = points.iter().filter(|p| **p != station).collect();

    // println!("{}", up);
}

#[test]
fn test_p1t1() {
    let input = ".#..#\n.....\n#####\n....#\n...##";
    let points = parse_input(&input);

    // println!("{:?}", points);

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 3, y: 4 };

    // println!("{} los {}: {}", p1, p2, line_of_sight(&points, p1, p2));
    assert!(!line_of_sight(&points, p1, p2));
}

#[test]
fn test_p1t2() {
    let input = ".#..#\n.....\n#####\n....#\n...##";
    let points = parse_input(&input);

    let max = best_station(&points);
    // println!("{:?}", max);
    assert_eq!(max.0, Point { x: 3, y: 4 });
    assert_eq!(max.1, 8);
}

#[test]
fn test_p2t1() {
    let input = ".#..#\n.....\n#####\n....#\n...##";
    let points = parse_input(&input);
    let station = Point { x: 3, y: 4 };
    let asteroids = points
        .iter()
        .copied()
        .filter(|p| *p != station)
        .collect::<Vec<_>>();

    let max = vaporize(&asteroids, station, 5);
    println!("{:?}", max);
}
