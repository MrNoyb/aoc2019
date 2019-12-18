use itertools::Itertools;
use std::collections::HashMap;

type Recipe = HashMap<Ingredient, Vec<Ingredient>>;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Ingredient {
    name: String,
    amount: u64,
}

impl Ingredient {
    fn new() -> Ingredient {
        Ingredient {
            name: "NONE".to_string(),
            amount: 0,
        }
    }
}

fn topological_sort(of: &str, fr: &str, from: &Recipe) -> Vec<Ingredient> {
    let wanted = match get_by_name(fr, from) {
        Some(i) => i,
        _ => return vec![],
    };
    // find the leafes of the graph
    let mut recipes = from.clone();
    let mut sorted_products = recipes
        .iter()
        .filter(|(_, v)| v.len() == 1 && v[0].name == of)
        .map(|(k, _)| k)
        .cloned()
        .collect::<Vec<_>>();

    sorted_products.iter().for_each(|ingr| {
        recipes.remove(ingr);
        ()
    });

    while !sorted_products.contains(&wanted) {
        let mut producable = recipes
            .iter()
            .filter(|(_, v)| {
                v.len() <= sorted_products.len()
                    && v.iter()
                        .all(|ingr| sorted_products.iter().any(|given| ingr.name == given.name))
            })
            .map(|(k, _)| k)
            .cloned()
            .collect::<Vec<_>>();

        producable.iter().for_each(|ingr| {
            recipes.remove(ingr);
            ()
        });
        sorted_products.append(&mut producable);
    }

    // println!("Sorted products:");
    // for p in &sorted_products {
    //     println!("-- {:?}", p);
    // }

    return sorted_products;
}

fn get_by_name(name: &str, rec: &Recipe) -> Option<Ingredient> {
    for k in rec.keys() {
        if k.name == name {
            return Some(k.clone());
        }
    }
    return None;
}

fn get_min_amount(of: &str, fr: &str, from: &Recipe) -> Option<Ingredient> {
    let mut product_chain = topological_sort(of, fr, from);

    let mut resolved: Vec<Ingredient> = vec![];
    while let Some(current) = product_chain.pop() {
        // !product_chain.is_empty() {
        // let current = product_chain.pop().unwrap();
        let mut reactants = from.get(&current).unwrap().clone();

        let mut scale = 1;
        if let Some(ind) = resolved.iter().position(|i| i.name == current.name) {
            let ingr = resolved.get(ind).unwrap();
            if ingr.amount > current.amount {
                scale = (ingr.amount as f64 / current.amount as f64).ceil() as u64;
            }
            resolved.remove(ind);
        }

        // println!("CURRENT:   {:?}", current);
        // println!("REACTANTS: {:?}", reactants);
        // println!("SCALE:     {}", scale);

        for react in reactants.drain(0..) {
            if let Some(ind) = resolved.iter().position(|i| i.name == react.name) {
                let ingr = resolved.get_mut(ind).unwrap();
                ingr.amount += react.amount * scale;
            } else {
                let mut react = react;
                react.amount *= scale;
                resolved.push(react);
            }
        }
        // println!("-> {:?}\n--", resolved);
    }

    // println!("RESOLVED: {:?}", resolved);

    if let Some(ind) = resolved.iter().position(|i| i.name == of) {
        let ingr = resolved.get(ind).unwrap();
        Some(ingr.clone())
    } else {
        None
    }
}

fn get_max_amount(of: &str, fr: &Ingredient, from: &Recipe) -> Option<Ingredient> {
    let low = 0;
    let high = fr.amount;

    fn bsearch(start: u64, end: u64, of: &str, fr: &str, rec: &Recipe) -> u64 {
        if start == end - 1 {
            return start;
        }
        let scale = (end - start) / 2 + start;

        let mut product_chain = topological_sort(of, fr, rec);

        let mut resolved: Vec<Ingredient> = vec![Ingredient {
            name: "FUEL".to_string(),
            amount: scale,
        }];
        while let Some(current) = product_chain.pop() {
            let mut reactants = rec.get(&current).unwrap().clone();

            let mut scale = 1;
            if let Some(ind) = resolved.iter().position(|i| i.name == current.name) {
                let ingr = resolved.get(ind).unwrap();
                if ingr.amount > current.amount {
                    scale = (ingr.amount as f64 / current.amount as f64).ceil() as u64;
                }
                resolved.remove(ind);
            }

            // println!("CURRENT:   {:?}", current);
            // println!("REACTANTS: {:?}", reactants);
            // println!("SCALE:     {}", scale);

            for react in reactants.drain(0..) {
                if let Some(ind) = resolved.iter().position(|i| i.name == react.name) {
                    let ingr = resolved.get_mut(ind).unwrap();
                    ingr.amount += react.amount * scale;
                } else {
                    let mut react = react;
                    react.amount *= scale;
                    resolved.push(react);
                }
            }
            // println!("-> {:?}\n--", resolved);
        }

        // println!("RESOLVED: {:?}", resolved);

        if let Some(ind) = resolved.iter().position(|i| i.name == of) {
            let ingr = resolved.get_mut(ind).unwrap();
            if ingr.amount > 1000000000000_u64 {
                bsearch(start, scale, of, fr, rec)
            } else {
                bsearch(scale, end, of, fr, rec)
            }
        } else {
            0
        }
    }

    Some(Ingredient {
        name: of.to_string(),
        amount: bsearch(low, high, &fr.name, of, from),
    })
}

fn parse_input(s: &str) -> HashMap<Ingredient, Vec<Ingredient>> {
    let mut map = HashMap::new();

    for line in s.lines() {
        let parts = line.split(" => ").collect::<Vec<_>>();
        let ops = parts[0];
        let prod = parts[1];

        let mut product = Ingredient::new();
        for (amount, name) in prod.split(' ').tuples() {
            let amount = amount.parse::<u64>().expect("Couldn't parse amount.");
            product = Ingredient {
                name: name.to_string(),
                amount,
            };
        }

        let mut v = vec![];
        for (amount, name) in ops.split(" ").tuples() {
            let amount = amount.parse::<u64>().expect("Couldn't parse amount.");
            v.push(Ingredient {
                name: name.trim_end_matches(',').to_string(),
                amount,
            });
        }

        map.insert(product, v);
    }

    return map;
}

fn main() {
    println!("--- Day 14: Space Stoichiometry ---\n");

    println!("Reading input...");

    let input = include_str!("../input");

    println!("Parsing input...");
    let ingrmap = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    if let Some(ingr) = get_min_amount("ORE", "FUEL", &ingrmap) {
        println!("Reaction needs {} amount of {}.", ingr.amount, ingr.name);
    } else {
        println!("No reaction found!");
    }

    println!("\n--- Part 2: ---\n");

    let cargo_ore = Ingredient {
        name: "ORE".to_string(),
        amount: 1000000000000_u64,
    };
    if let Some(ingr) = get_max_amount("FUEL", &cargo_ore, &ingrmap) {
        println!("{} amount of {} can be produced.", ingr.amount, ingr.name);
    } else {
        println!("No fuel can be produced!");
    }
}

#[test]
fn test1() {
    let input = r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    let recipe = parse_input(&input);

    for (k, v) in &recipe {
        println!("{:?} : {:?}", k, v);
    }

    assert_eq!(
        Some(Ingredient {
            name: "FUEL".to_string(),
            amount: 1
        }),
        get_by_name("FUEL", &recipe)
    );

    assert_eq!(
        Some(Ingredient {
            name: "ORE".to_string(),
            amount: 31
        }),
        get_min_amount("ORE", "FUEL", &recipe)
    );
}

#[test]
fn test2() {
    let input = r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    let recipe = parse_input(&input);

    let ore = get_min_amount("ORE", "FUEL", &recipe).unwrap();
    let cargo_ore = Ingredient {
        name: "ORE".to_string(),
        amount: 1000000000000_u64,
    };
    let fuel = get_max_amount("FUEL", &cargo_ore, &recipe).unwrap();

    println!("TEST2: {:?}", fuel);

    assert_eq!(
        ore,
        Ingredient {
            name: "ORE".to_string(),
            amount: 13312
        },
    );

    assert_eq!(fuel.amount, 82892753);
}

#[test]
fn test3() {
    let input = r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    let recipe = parse_input(&input);

    let ore = get_min_amount("ORE", "FUEL", &recipe).unwrap();

    let cargo_ore = Ingredient {
        name: "ORE".to_string(),
        amount: 1000000000000_u64,
    };
    let fuel = get_max_amount("FUEL", &cargo_ore, &recipe).unwrap();

    println!(
        "TEST3: {:?}",
        get_max_amount("FUEL", &cargo_ore, &recipe).unwrap()
    );

    assert_eq!(
        ore,
        Ingredient {
            name: "ORE".to_string(),
            amount: 2210736
        }
    );

    assert_eq!(fuel.amount, 460664);
}
