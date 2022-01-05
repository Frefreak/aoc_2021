use std::cmp::{max, min};

#[derive(Debug)]
struct Line {
    from: (u32, u32),
    to: (u32, u32),
}

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .lines()
        .map(|l| {
            let pts: Vec<_> = l.split(" -> ").collect();
            let p1 = pts[0]
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let p2 = pts[1]
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            Line {
                from: (p1[0], p1[1]),
                to: (p2[0], p2[1]),
            }
        })
        .collect::<Vec<_>>();
    let lines: Vec<_> = lines
        .iter()
        .filter(|l| {
            l.from.0 == l.to.0
                || l.from.1 == l.to.1
                || (l.from.0 as i32 - l.to.0 as i32).abs()
                    == (l.from.1 as i32 - l.to.1 as i32).abs()
        })
        .collect();

    let mut maxx = 0;
    let mut maxy = 0;
    for l in lines.iter() {
        if l.from.0 > maxx {
            maxx = l.from.0;
        }
        if l.to.0 > maxx {
            maxx = l.to.0;
        }
        if l.from.1 > maxy {
            maxy = l.from.1;
        }
        if l.to.1 > maxy {
            maxy = l.to.1;
        }
    }
    println!("{} {}", maxx, maxy);
    let mut grid = vec![vec![0; (maxx + 1) as usize]; (maxy + 1) as usize];
    for l in lines.iter() {
        if l.from.0 == l.to.0 {
            // x equal
            let mi = min(l.from.1, l.to.1);
            let ma = max(l.from.1, l.to.1);
            for i in mi..ma + 1 {
                grid[i as usize][l.from.0 as usize] += 1;
            }
        } else if l.from.1 == l.to.1 {
            // y equal
            let mi = min(l.from.0, l.to.0);
            let ma = max(l.from.0, l.to.0);
            for i in mi..ma + 1 {
                grid[l.from.1 as usize][i as usize] += 1;
            }
        } else {
            let stepx = {
                if l.from.0 > l.to.0 {
                    -1
                } else {
                    1
                }
            };
            let stepy = {
                if l.from.1 > l.to.1 {
                    -1
                } else {
                    1
                }
            };
            let (mut sx, mut sy) = l.from;
            grid[sy as usize][sx as usize] += 1;
            for _i in 0..(l.to.0 as i32 - l.from.0 as i32).abs() {
                sx = (sx as i32 + stepx) as u32;
                sy = (sy as i32 + stepy) as u32;
                grid[sy as usize][sx as usize] += 1;
            }
        }
    }
    let mut p = 0;
    for row in grid.iter() {
        for cell in row.iter() {
            if *cell >= 2 {
                p += 1;
            }
        }
    }
    println!("{}", p);
}
