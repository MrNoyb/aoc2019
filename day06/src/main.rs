use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Read;

fn parent_orbits(om: &HashMap<String, Option<String>>, p: &str) -> Vec<String> {
    let mut v = vec![];

    if let Some(parent) = om.get(p) {
        let mut tp = parent;
        while tp.is_some() {
            let name = tp.as_ref().unwrap();
            v.push(name.to_string());
            tp = om.get(name).unwrap();
        }
    }

    return v;
}

fn total_orbits(om: &HashMap<String, Option<String>>, p: &str) -> u32 {
    if let Some(parent) = om.get(p) {
        match parent {
            None => 0,
            Some(n) => 1 + total_orbits(om, n),
        }
    } else {
        return 0;
    }
}

fn parse_input(s: &str) -> HashMap<String, Option<String>> {
    let mut m = HashMap::new();

    for line in s.lines() {
        let mut it = line.split(')');
        let parent = it.next().unwrap();
        let child = it.next().unwrap();
        m.insert(child.to_string(), Some(parent.to_string()));
        if !m.contains_key(parent) {
            m.insert(parent.to_string(), None);
        }
    }

    return m;
}

fn main() {
    println!("--- Day 4: Secure Container ---\n");

    println!("Reading input...");

    let mut input = String::new();
    {
        let mut file = File::open("input").expect("Could not open input file.");
        file.read_to_string(&mut input)
            .expect("Could not read from input file.");
    }

    println!("Parsing input...");
    let om = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    let res: u32 = om.keys().map(|s| total_orbits(&om, s)).sum();
    println!("Total number of orbits: {}", res);

    println!("\n--- Part 2: ---\n");

    let you_parents = parent_orbits(&om, "YOU");
    let san_parents = parent_orbits(&om, "SAN");

    // find common parent
    let common_parent = {
        let mut res = "";
        for p in &you_parents {
            if san_parents.iter().find(|e| *e == p).is_some() {
                res = p;
                break;
            }
        }
        res
    };

    let d1 = you_parents
        .iter()
        .take_while(|s| *s != common_parent)
        .count();
    let d2 = san_parents
        .iter()
        .take_while(|s| *s != common_parent)
        .count();

    println!("Common parent: {}", common_parent);

    let res = d1 + d2;
    println!("Minimum orbital transfers: {}", res);
}

#[test]
fn test_p1() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    let om = parse_input(&input);

    println!("{:?}", om);

    assert_eq!(total_orbits(&om, "COM"), 0);
    assert_eq!(total_orbits(&om, "D"), 3);
    assert_eq!(total_orbits(&om, "L"), 7);
}

#[test]
fn test_p2() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    let om = parse_input(&input);

    let vec: Vec<String> = vec![];
    assert_eq!(parent_orbits(&om, "COM"), vec);
    assert_eq!(parent_orbits(&om, "D"), vec!["C", "B", "COM"]);
    assert_eq!(
        parent_orbits(&om, "L"),
        vec!["K", "J", "E", "D", "C", "B", "COM"]
    );
}

#[test]
fn test_p3() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    let om = parse_input(&input);

    let exp_common_parent = String::from("D");
    let exp_distance = 4;

    let you_parents = parent_orbits(&om, "YOU");
    let san_parents = parent_orbits(&om, "SAN");

    println!("you: {:?}", you_parents);
    println!("san: {:?}", san_parents);

    // find common parent
    let common_parent = {
        let mut res = "";
        for p in &you_parents {
            if san_parents.iter().find(|e| *e == p).is_some() {
                res = p;
                break;
            }
        }
        res
    };

    let d1 = you_parents
        .iter()
        .take_while(|s| *s != common_parent)
        .count();
    let d2 = san_parents
        .iter()
        .take_while(|s| *s != common_parent)
        .count();

    let distance = d1 + d2;

    assert_eq!(common_parent, exp_common_parent);
    assert_eq!(distance, exp_distance);
}
