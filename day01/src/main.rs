use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_p1t1() {
    let mass = 12;
    let fuel = 2;

    assert_eq!(fuel, calc_fuel(&mass));
}

#[test]
fn test_p1t2() {
    let mass = 14;
    let fuel = 2;

    assert_eq!(fuel, calc_fuel(&mass));
}

#[test]
fn test_p1t3() {
    let mass = 1969;
    let fuel = 654;

    assert_eq!(fuel, calc_fuel(&mass));
}

#[test]
fn test_p1t4() {
    let mass = 100756;
    let fuel = 33583;

    assert_eq!(fuel, calc_fuel(&mass));
}

#[test]
fn test_p2t1() {
    let mass = 14;
    let fuel = 2;

    assert_eq!(fuel, calc_fuel(&mass));
}

#[test]
fn test_p2t2() {
    let mass = 1969;
    let fuelfuel = 966;

    assert_eq!(fuelfuel, calc_fuelfuel(&mass));
}

#[test]
fn test_p2t3() {
    let mass = 100756;
    let fuelfuel = 50346;

    assert_eq!(fuelfuel, calc_fuelfuel(&mass));
}

fn parse_input(s: &str) -> Vec<i32> {
    let mut v = vec![];
    for line in s.lines() {
        if line.len() == 0 {
            continue;
        }
        let n = match line.parse::<i32>() {
            Err(e) => panic!("Could not parse {}: {}", line, e),
            Ok(i) => i,
        };
        v.push(n);
    }

    return v;
}

fn calc_fuel(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn calc_fuelfuel(mass: &i32) -> i32 {
    let mut sum = 0;
    let mut i = calc_fuel(&mass);
    while i > 0 {
        sum += i;
        i = calc_fuel(&i);
    }
    return sum;
}

fn main() {
    println!("--- Day 1: The Tyranny of the Rocket Equation ---\n");

    println!("Reading input from file...");
    let mut input = String::new();

    {
        let mut file = File::open("input").expect("Could not open file");

        file.read_to_string(&mut input)
            .expect("Could not read file");
    }

    println!("Parsing input...");
    let input = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    let mut sum = 0;
    for m in &input {
        sum += calc_fuel(&m);
    }

    println!("Total fuel cost: {}", sum);

    println!("\n--- Part 2: ---\n");

    let mut sum2 = 0;
    for m in &input {
        sum2 += calc_fuelfuel(&m);
    }

    println!("Total fuel cost: {}", sum2);
}
