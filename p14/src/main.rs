use std::collections::HashMap;

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut lines = content.lines();
    let template = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    let mut instructions = HashMap::new();
    while let Some(instruction) = lines.next() {
        let mut segs = instruction.split(" -> ");
        let f = segs.next().unwrap().to_string();
        let t = segs.next().unwrap().chars().nth(0).unwrap();
        instructions.insert(f, t);
    }
    let mut storage: HashMap<String, u64> = HashMap::new();
    for i in 0..template.len() - 1 {
        let sub = format!("{}{}", template[i], template[i+1]);
        storage.insert(sub, 1);
    }

    let mut first = format!("{}{}", template[0], template[1]);
    let mut last = format!("{}{}", template[template.len()-2], template[template.len()-1]);
    for _i in 0..40 {
        if instructions.contains_key(&first) {
            first = format!("{}{}", first.chars().nth(0).unwrap(), instructions[&first]);
        }
        if instructions.contains_key(&last) {
            last = format!("{}{}", instructions[&last], last.chars().nth(1).unwrap());
        }
        let mut new = HashMap::new();
        for (k, v) in storage.iter() {
            if instructions.contains_key(k) {
                let mut kchars = k.chars();
                let char = instructions[k];
                let p1 = format!("{}{}", kchars.next().unwrap(), char);
                let p2 = format!("{}{}", char, kchars.next().unwrap());
                let ep1 = new.entry(p1).or_insert(0);
                *ep1 += v;
                let ep2 = new.entry(p2).or_insert(0);
                *ep2 += v;
            }
        }
        storage = new;
        // println!("{:?}", storage);
    }
    let mut count = HashMap::new();
    for (k, v) in storage.iter() {
        let p1 = k.chars().nth(0).unwrap();
        let p2 = k.chars().nth(1).unwrap();
        if *k == first {
            let ep1 = count.entry(p1).or_insert(0);
            *ep1 += v - 1;
            let ep2 = count.entry(p2).or_insert(0);
            *ep2 += v;
        } else if *k == last {
            let ep1 = count.entry(p1).or_insert(0);
            *ep1 += v;
            let ep2 = count.entry(p2).or_insert(0);
            *ep2 += v - 1;
        } else {
            let ep1 = count.entry(p1).or_insert(0);
            *ep1 += v;
            let ep2 = count.entry(p2).or_insert(0);
            *ep2 += v;
        }
    }
    for (_, v) in count.iter_mut() {
        *v = *v / 2;
    }
    let e = count.entry(first.chars().nth(0).unwrap()).or_insert(0);
    *e += 1;
    let e = count.entry(last.chars().nth(1).unwrap()).or_insert(0);
    *e += 1;

    println!("{}", count.values().max().unwrap() - count.values().min().unwrap());
}
