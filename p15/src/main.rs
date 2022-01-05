use std::collections::{HashSet, VecDeque, HashMap};

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut grid: Vec<Vec<_>> = content.lines().map(|x| {
        x.chars().map(|y| y.to_digit(10).unwrap()).collect()
    }).collect();

    // expand grid
    for l in grid.iter_mut() {
        let mut arr = vec![];
        arr.push(l.clone());
        for _i in 0..4 {
            let newl: Vec<u32> = l.iter().map(|x| x % 9 + 1).collect();
            arr.push(newl.clone());
            *l = newl;
        }
        *l = arr.concat();
    }
    let mut arr = vec![];
    arr.push(grid.clone());
    for _i in 0..4 {
        let mut newgrid = grid.clone();
        for l in newgrid.iter_mut() {
            for i in l.iter_mut() {
                *i = *i % 9 + 1;
            }
        }
        grid = newgrid.clone();
        arr.push(newgrid);
    }
    grid = arr.concat();
    bfs(&grid);
}

fn bfs(grid: &Vec<Vec<u32>>) {
    let mut result_cost = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    result_cost.insert((0, 0), 0);

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        let cur_cost = result_cost[&(y, x)];
        // up
        if y > 0 {
            let new_cost = grid[y-1][x] + cur_cost;
            let coord = (y-1, x);
            if !result_cost.contains_key(&coord) || result_cost[&coord] > new_cost {
                result_cost.insert(coord, new_cost);
                queue.push_back(coord);
            }
        }
        // left
        if x > 0 {
            let new_cost = grid[y][x-1] + cur_cost;
            let coord = (y, x-1);
            if !result_cost.contains_key(&coord) || result_cost[&coord] > new_cost {
                let coord = coord;
                result_cost.insert(coord, new_cost);
                queue.push_back(coord);
            }
        }
        // right
        if x < grid[0].len()-1 {
            let new_cost = grid[y][x+1] + cur_cost;
            let coord = (y, x+1);
            if !result_cost.contains_key(&coord) || result_cost[&coord] > new_cost {
                let coord = coord;
                result_cost.insert(coord, new_cost);
                queue.push_back(coord);
            }
        }
        // down
        if y < grid.len()-1 {
            let new_cost = grid[y+1][x] + cur_cost;
            let coord = (y+1, x);
            if !result_cost.contains_key(&coord) || result_cost[&coord] > new_cost {
                let coord = coord;
                result_cost.insert(coord, new_cost);
                queue.push_back(coord);
            }
        }
    }
    // println!("{:?}", result_cost);
    let dest = (grid.len()-1, grid[0].len()-1);
    println!("{}", result_cost[&dest]);

}
