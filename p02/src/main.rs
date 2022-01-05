use std::fs;

fn main() {
    let mut hori = 0;
    let mut depth = 0;
    let mut aim = 0;
    for l in fs::read_to_string("./input.txt").unwrap().lines() {
        let val = l.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
        if l.starts_with("forward") {
            hori += val;
            depth += aim * val;
        } else if l.starts_with("down") {
            // depth += val;
            aim += val;
        } else if l.starts_with("up") {
            // depth -= val;
            aim -= val;
        }
    }
    println!("{} {} {}", hori, depth, hori * depth);
}
