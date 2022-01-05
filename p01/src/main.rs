use std::fs;

fn main() {
    let nums_str = fs::read_to_string("./input.txt").unwrap();
    let mut inc = 0;
    let nums: Vec<u32> = nums_str.lines().map(|x| x.parse().unwrap()).collect();
    let mut prev = nums[0] + nums[1] + nums[2];
    for i in 3..nums.len() {
        let k = nums[i-2] + nums[i-1] + nums[i];
        if k > prev {
            inc += 1;
        }
        prev = k;
    }
    println!("{}", inc);
}
