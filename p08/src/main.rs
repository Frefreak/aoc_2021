use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let mut total = 0;
    for l in content.lines() {
        let result = analyze(l);
        println!("{:?}", result);
        total += result.unwrap();
    }
    println!("{}", total);
}

// +---1---+
// |       |
// 2       3
// |       |
// +---4---+
// |       |
// 5       6
// |       |
// +---7---+

fn valid(maps: &HashMap<char, char>, seq: &str) -> Option<u8> {
    let mut hs = Vec::new();
    for ch in seq.chars() {
        hs.push(maps[&ch]);
    }
    hs.sort();
    let rep: String = hs.iter().collect();
    match rep.as_ref() {
        "123567" => Some(0),
        "36" => Some(1),
        "13457" => Some(2),
        "13467" => Some(3),
        "2346" => Some(4),
        "12467" => Some(5),
        "124567" => Some(6),
        "136" => Some(7),
        "1234567" => Some(8),
        "123467" => Some(9),
        _ => None,
    }
}

fn analyze(l: &str) -> Option<u32> {
    let mut splits = l.split(" | ");
    let part1 = splits.next().unwrap();
    let part2 = splits.next().unwrap();
    let mut test_subjects = part1.split_whitespace().collect::<Vec<_>>();
    let outputs = part2.split_whitespace().collect::<Vec<_>>();
    test_subjects.extend(&outputs);
    let mut maps = HashMap::new();
    for perm in "abcdefg".chars().permutations(7).unique() {
        for (i, ch) in perm.into_iter().enumerate() {
            maps.insert(ch, char::from_digit((i+1) as u32, 10).unwrap());
        }
        if test_subjects.iter().all(|x| valid(&maps, x).is_some()) {
            let mut n = 0;
            for comb in outputs.iter() {
                n = n * 10 + valid(&maps, comb).unwrap() as u32;
            }
            return Some(n);
        }
    }
    None
}
