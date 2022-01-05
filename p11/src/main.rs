use std::collections::HashMap;

fn print_grid(grid: &Vec<Vec<u32>>) {
    for line in grid.iter() {
        for ch in line {
            print!("{}", ch);
        }
        println!("");
    }
    println!("");
}

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut grid: Vec<Vec<u32>> = content.lines().map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect()).collect();
    let total: u32 = grid.iter().map(|x| x.len() as u32).sum();
    println!("total: {}", total);
    let mut flash = 0;
    for _i in 0..100 {
        flash += step(&mut grid);
    }
    println!("{}", flash);
    let mut i = 100;
    loop {
        let this_flash = step(&mut grid);
        flash += this_flash;
        if total as u32 == this_flash {
            println!("{}", i+1);
            break;
        }
        i += 1;
    }
}

fn step(grid: &mut Vec<Vec<u32>>) -> u32 {
    let mut light_map: HashMap<(usize, usize), u32> = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            light_map.insert((r, c), 1);
        }
    }
    let mut prev_flash = 0;
    loop {
        let mut cnt = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] + light_map[&(r, c)] >= 10 {
                    cnt += 1;
                    continue;
                }
                let mut local_cnt = 0;
                for i in [0, -1 as i32, 1] {
                    for j in [0, -1 as i32, 1] {
                        let r = r as i32;
                        let c = c as i32;
                        if i != 0 || j != 0 {
                            if r + i >= 0 && r + i < grid.len() as i32 && c + j >= 0 && c + j < grid[0].len() as i32 {
                                let (x, y) = ((r+i) as usize, (c+j) as usize);
                                if grid[x][y] + light_map[&(x, y)] >= 10 {
                                    local_cnt += 1;
                                }
                            }
                        }
                    }
                }
                light_map.insert((r, c), 1 + local_cnt);
                if grid[r][c] + light_map[&(r, c)] >= 10 {
                    cnt += 1;
                }
            }
        }
        if cnt == prev_flash {
            break;
        }
        prev_flash = cnt;
    }
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            grid[r][c] += light_map[&(r, c)];
            if grid[r][c] >= 10 {
                grid[r][c] = 0;
            }
        }
    }
    return prev_flash;
}
