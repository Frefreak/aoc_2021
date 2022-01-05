use std::collections::HashSet;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<u32>> = content
        .lines()
        .map(|x| x.chars().map(|y| char::to_digit(y, 10).unwrap()).collect())
        .collect();
    // println!("{:?}", grid);
    let mut risk = 0;
    let width = grid[0].len();
    let height = grid.len();
    println!("height: {}, width: {}", height, width);
    for (i, r) in grid.iter().enumerate() {
        for (j, ch) in r.iter().enumerate() {
            let mut is_low = true;
            if i > 0 && grid[i - 1][j] <= *ch {
                is_low = false;
            }
            if i < height - 1 && grid[i + 1][j] <= *ch {
                is_low = false;
            }
            if j > 0 && grid[i][j - 1] <= *ch {
                is_low = false;
            }
            if j < width - 1 && grid[i][j + 1] <= *ch {
                is_low = false;
            }
            if is_low {
                println!("({},{}) {}", i, j, ch + 1);
                risk += ch + 1;
            }
        }
    }
    println!("risk: {}", risk);

    let mut unexplored: HashSet<(usize, usize)> = HashSet::new();
    for (i, _) in grid.iter().enumerate() {
        for j in 0..width {
            if grid[i][j] != 9 {
                unexplored.insert((i, j));
            }
        }
    }
    let mut idx = 0;
    let mut blobs = vec![];
    while !unexplored.is_empty() {
        let starting = *unexplored.iter().next().unwrap();
        let mut size = 0_u32;
        dfs(&mut unexplored, &starting, &mut size, width, height);
        println!("blob {}: {}", idx, size);
        blobs.push(size);
        idx += 1;
    }
    blobs.sort_unstable();
    println!("{}", blobs[idx-3] * blobs[idx-2] * blobs[idx-1]);
}

fn dfs(
    unexplored: &mut HashSet<(usize, usize)>,
    start @ (i, j): &(usize, usize),
    size: &mut u32,
    width: usize,
    height: usize,
) {
    unexplored.remove(start);
    *size += 1;
    if *i > 0 && unexplored.contains(&(*i - 1, *j)) {
        dfs(unexplored, &(*i-1, *j), size, width, height);
    }
    if *j > 0 && unexplored.contains(&(*i, *j - 1)) {
        dfs(unexplored, &(*i, *j-1), size, width, height);
    }
    if *i < height - 1 && unexplored.contains(&(*i+1, *j)) {
        dfs(unexplored, &(*i+1, *j), size, width, height);
    }
    if *j < width - 1 && unexplored.contains(&(*i, *j+1)) {
        dfs(unexplored, &(*i, *j+1), size, width, height);
    }
}
