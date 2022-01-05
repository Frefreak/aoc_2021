use std::collections::HashSet;

fn print_grid(width: usize, height: usize, east_fish: &HashSet<(usize, usize)>, south_fish: &HashSet<(usize, usize)>) {
    let mut grid = vec![];
    for _ in 0..height {
        grid.push(vec!['.'; width]);
    }
    for (i, j) in east_fish {
        grid[*i][*j] = '>';
    }
    for (i, j) in south_fish {
        grid[*i][*j] = 'v';
    }
    for l in grid.iter() {
        println!("{}", l.iter().collect::<String>());
    }
}

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut grid = vec![];
    for l in content.lines() {
        grid.push(l.chars().collect::<Vec<_>>());
    }
    let height = grid.len();
    let width = grid[0].len();
    let mut east_fish = HashSet::new();
    let mut south_fish = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '>' {
                east_fish.insert((i, j));
            } else if *ch == 'v' {
                south_fish.insert((i, j));
            }
        }
    }
    print_grid(width, height, &east_fish, &south_fish);

    let mut cnt = 1;
    loop {
        if step(width, height, &mut east_fish, &mut south_fish) {
            println!("stopped at {}", cnt);
            break;
        }
        // println!("------------ {} -------------", cnt);
        // print_grid(width, height, &east_fish, &south_fish);
        cnt += 1;
    }

}

fn step(width: usize, height: usize, east_fish: &mut HashSet<(usize, usize)>, south_fish: &mut HashSet<(usize, usize)>) -> bool {
    let mut east_move = vec![];
    let mut south_move = vec![];
    for fish@(i, j) in east_fish.iter() {
        let new_pos = (*i, (j+1) % width);
        if !east_fish.contains(&new_pos) && !south_fish.contains(&new_pos) {
            east_move.push(*fish);
        }
    }
    for fish@(i, j) in east_move.iter() {
        east_fish.remove(&fish);
        east_fish.insert((*i, (j+1) % width));
    }
    for fish@(i, j) in south_fish.iter() {
        let new_pos = ((i+1) % height, *j);
        if !east_fish.contains(&new_pos) && !south_fish.contains(&new_pos) {
            south_move.push(*fish);
        }
    }
    for fish@(i, j) in south_move.iter() {
        south_fish.remove(&fish);
        south_fish.insert(((i+1) % height, *j));
    }
    east_move.is_empty() && south_move.is_empty()
}
