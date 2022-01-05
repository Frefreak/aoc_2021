use std::collections::HashMap;

fn main() {
    let mut fish: Vec<u8> = vec![5,1,2,1,5,3,1,1,1,1,1,2,5,4,1,1,1,1,2,1,2,1,1,1,1,1,2,1,5,1,1,1,3,1,1,1,3,1,1,3,1,1,4,3,1,1,4,1,1,1,1,2,1,1,1,5,1,1,5,1,1,1,4,4,2,5,1,1,5,1,1,2,2,1,2,1,1,5,3,1,2,1,1,3,1,4,3,3,1,1,3,1,5,1,1,3,1,1,4,4,1,1,1,5,1,1,1,4,4,1,3,1,4,1,1,4,5,1,1,1,4,3,1,4,1,1,4,4,3,5,1,2,2,1,2,2,1,1,1,2,1,1,1,4,1,1,3,1,1,2,1,4,1,1,1,1,1,1,1,1,2,2,1,1,5,5,1,1,1,5,1,1,1,1,5,1,3,2,1,1,5,2,3,1,2,2,2,5,1,1,3,1,1,1,5,1,4,1,1,1,3,2,1,3,3,1,3,1,1,1,1,1,1,1,2,3,1,5,1,4,1,3,5,1,1,1,2,2,1,1,1,1,5,4,1,1,3,1,2,4,2,1,1,3,5,1,1,1,3,1,1,1,5,1,1,1,1,1,3,1,1,1,4,1,1,1,1,2,2,1,1,1,1,5,3,1,2,3,4,1,1,5,1,2,4,2,1,1,1,2,1,1,1,1,1,1,1,4,1,5];
    // let fish: Vec<u8> = vec![3, 4, 3, 1, 2];
    let mut fish_hash: HashMap<u8, u64> = HashMap::new();
    for n in fish.iter() {
        let ent = fish_hash.get_mut(&n);
        match ent {
            Some(nn) => {*nn += 1;},
            None => {fish_hash.insert(*n, 1);},
        }
    }
    println!("{:?}", fish_hash);
    for _i in 0..256 {
        let mut new_fish_hash: HashMap<u8, u64> = HashMap::new();
        for (n, num) in fish_hash.iter() {
            if *n == 0 {
                let existing = new_fish_hash.get(&6).unwrap_or(&0).clone();
                new_fish_hash.insert(6, existing + *num);
                new_fish_hash.insert(8, *num);
            } else {
                let existing = new_fish_hash.get(&(*n-1)).unwrap_or(&0).clone();
                new_fish_hash.insert(*n - 1, existing + *num);
            }
        }
        fish_hash = new_fish_hash;
        // println!("day {}: {:?}", i+1, fish_hash);
        // println!("{:?}", fish);
    }
    let mut cnt = 0;
    for (_, num) in fish_hash.iter() {
        cnt += num;
    }
    println!("{}", cnt);
}
