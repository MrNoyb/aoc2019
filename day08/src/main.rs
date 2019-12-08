use image::bmp::BMPEncoder;
use image::ColorType;
use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;

type Layer = Vec<u8>;

const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;

fn layers2pixels(layers: &[Layer]) -> [u8; (WIDTH * HEIGHT) as usize] {
    let mut v = [99; (WIDTH * HEIGHT) as usize];
    let mut done = 0;

    'outer: for l in layers {
        for (i, val) in l.iter().enumerate() {
            if v[i] != 99 {
                continue;
            }
            match val {
                0 => {
                    v[i] = 0;
                    done += 1;
                }
                1 => {
                    v[i] = 255;
                    done += 1;
                }
                _ => continue,
            }
            if done == WIDTH * HEIGHT {
                break 'outer;
            }
        }
    }

    return v;
}

fn pixel2utf8(pixels: &[u8]) -> String {
    let mut s = String::new();

    for (i, p) in pixels.iter().enumerate() {
        if *p == 0 {
            s.push(' ');
        } else {
            s.push('\u{2588}');
        }
        if (i + 1) % WIDTH as usize == 0 {
            s.push('\n');
        }
    }
    return s;
}

fn parse_input(s: &str) -> Vec<Layer> {
    let mut v = vec![];

    let mut cur_layer = vec![];

    for (i, c) in s.trim().chars().enumerate() {
        let n = c.to_digit(10).unwrap() as u8;
        cur_layer.push(n);
        if (i + 1) % (WIDTH * HEIGHT) as usize == 0 {
            v.push(cur_layer);
            cur_layer = Vec::new();
        }
    }
    return v;
}

fn main() {
    println!("--- Day 8: Space Image Format ---\n");

    println!("Reading input...");

    let mut input = String::new();
    {
        let mut file = File::open("input").expect("Could not open input file.");
        file.read_to_string(&mut input)
            .expect("Could not read from input file.");
    }

    println!("Parsing input...");
    let layers = parse_input(&input);

    println!("\n--- Part 1: ---\n");

    let mut zeroes = vec![];
    for l in &layers {
        let count_zero = l.iter().filter(|x| **x == 0).count();
        zeroes.push(count_zero);
    }
    let (ind, _) = zeroes
        .iter()
        .enumerate()
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    let count_ones = layers[ind].iter().filter(|x| **x == 1).count();
    let count_twos = layers[ind].iter().filter(|x| **x == 2).count();

    println!("Checksum: {}", count_ones * count_twos);

    println!("\n--- Part 2: ---\n");

    let path = Path::new("output.bmp");
    let mut picfile = File::create(path).unwrap();

    let mut encoder = BMPEncoder::new(&mut picfile);
    let pixels = layers2pixels(&layers);

    encoder
        .encode(&pixels, WIDTH, HEIGHT, ColorType::Gray(8))
        .expect("Could not write image.");

    println!("Image written to {}", path.display());
    println!("\nText image representation:\n\n{}", pixel2utf8(&pixels));
}
