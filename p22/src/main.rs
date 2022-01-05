use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Cuboid {
    st: State,
    xr: (i32, i32),
    yr: (i32, i32),
    zr: (i32, i32),
}

fn parse_line(s: &str) -> Cuboid {
    let mut iter = s.split_whitespace();
    let st = iter.next().unwrap();
    let state = if st == "on" { State::On } else { State::Off };
    let mut seg_iter = iter.next().unwrap().split(",");
    let xr = seg_iter
        .nth(0)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .split("..")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let yr = seg_iter
        .nth(0)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .split("..")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let zr = seg_iter
        .nth(0)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .split("..")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    Cuboid {
        st: state,
        xr: (xr[0], xr[1]),
        yr: (yr[0], yr[1]),
        zr: (zr[0], zr[1]),
    }
}

fn intersection(c1: &Cuboid, c2: &Cuboid) -> Option<Cuboid> {
    let x1 = max(c1.xr.0, c2.xr.0);
    let x2 = min(c1.xr.1, c2.xr.1);
    if x1 > x2 {
        return None;
    }
    let y1 = max(c1.yr.0, c2.yr.0);
    let y2 = min(c1.yr.1, c2.yr.1);
    if y1 > y2 {
        return None;
    }
    let z1 = max(c1.zr.0, c2.zr.0);
    let z2 = min(c1.zr.1, c2.zr.1);
    if z1 > z2 {
        return None;
    }
    if c1.st == State::On {
        return Some(Cuboid {
            st: State::Off,
            xr: (x1, x2),
            yr: (y1, y2),
            zr: (z1, z2),
        });
    } else {
        return Some(Cuboid {
            st: State::On,
            xr: (x1, x2),
            yr: (y1, y2),
            zr: (z1, z2),
        });
    }
}

fn solve(cuboids: &Vec<Cuboid>) {
    let mut existing = vec![];
    for cuboid in cuboids {
        let mut cc = vec![];
        if cuboid.st == State::On {
            cc.push(cuboid.clone());
        }
        for c in &existing {
            if let Some(r) = intersection(c, cuboid) {
                cc.push(r);
            }
        }
        existing.extend(cc);
        // println!("{:?}", existing);
    }
    let mut total: i64 = 0;
    for c in existing {
        let s = (c.xr.1 - c.xr.0 + 1) as i64 * (c.yr.1 - c.yr.0 + 1) as i64 * (c.zr.1 - c.zr.0 + 1) as i64;
        if c.st == State::On {
            total += s;
        } else {
            total -= s;
        }
    }
    println!("{}", total);
}

fn dumb(cuboids: &Vec<Cuboid>) {
    let mut hs = HashSet::new();
    for cuboid in cuboids {
        for x in cuboid.xr.0..=cuboid.xr.1 {
            for y in cuboid.yr.0..=cuboid.yr.1 {
                for z in cuboid.zr.0..=cuboid.zr.1 {
                    if cuboid.st == State::On {
                        hs.insert((x, y, z));
                    } else {
                        hs.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    println!("dumb: {}", hs.len());
}

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let cuboids = content.lines().map(|l| parse_line(l)).collect::<Vec<_>>();
    let simple_cuboids = cuboids
        .clone()
        .into_iter()
        .filter(|c| {
            c.xr.0 >= -50
                && c.xr.1 <= 50
                && c.yr.0 >= -50
                && c.yr.1 <= 50
                && c.zr.0 >= -50
                && c.zr.1 <= 50
        })
        .collect::<Vec<_>>();
    // println!("{}", simple_cuboids.len());
    // let t = simple_cuboids.into_iter().take(20).collect::<Vec<_>>();
    // dumb(&t);
    solve(&simple_cuboids);
    solve(&cuboids);
}
