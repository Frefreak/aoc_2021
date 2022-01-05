fn print_grid(grid: &Vec<Vec<char>>) {
    println!("{} x {}", grid.len(), grid[0].len());
    for l in grid.iter() {
        println!("{}", l.iter().collect::<String>());
    }
}

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let coords = content.split("\n\n").nth(0).unwrap().lines().map(|x| {
        let coordx = x.split(",").nth(0).unwrap().parse::<u32>().unwrap();
        let coordy = x.split(",").nth(1).unwrap().parse::<u32>().unwrap();
        (coordx, coordy)
    }).collect::<Vec<_>>();
    let maxx = coords.iter().map(|x| x.0).max().unwrap() as usize;
    let maxy = coords.iter().map(|x| x.1).max().unwrap() as usize;
    let mut grid = vec![vec!['.'; maxx+1]; maxy+1];
    for coord in coords.iter() {
        grid[coord.1 as usize][coord.0 as usize] = '#';
    }
    print_grid(&grid);

    let steps = content.split("\n\n").nth(1).unwrap().lines().map(|x| {
        let mut segs = x[11..x.len()].split("=");
        let dir = segs.next().unwrap();
        let pos = segs.next().unwrap().parse::<usize>().unwrap();
        (dir, pos)
    }).collect::<Vec<_>>();
    // println!("----");
    do_fold(&mut grid, &steps[0]);
    println!("{}", grid.iter().map(|l| l.iter().filter(|ch| **ch == '#').count()).sum::<usize>());

    for step in steps.iter().skip(1) {
        do_fold(&mut grid, step);
    }
    print_grid(&grid);

    // print_grid(&grid);
    // println!("----");
    // do_fold(&mut grid, steps[1]);
    // print_grid(&grid);
}

fn do_fold(grid: &mut Vec<Vec<char>>, step: &(&str, usize)) {
    match step.0 {
        "y" => {
            for j in step.1..grid.len() {
                for (i, ch) in std::mem::take(&mut grid[j]).iter().enumerate() {
                    if *ch == '#' {
                        let diff = j - step.1;
                        grid[step.1 - diff][i] = '#';
                    }
                }
            }
            grid.truncate(step.1);
        },
        "x" => {
            for j in 0..grid.len() {
                for i in step.1..grid[0].len() {
                    if grid[j][i] == '#' {
                        let diff = i - step.1;
                        grid[j][step.1 - diff] = '#';
                    }
                }
            }
            for row in grid.iter_mut() {
                row.truncate(step.1);
            }
        },
        _ => {
            panic!("impossible");
        }
    }
}
