fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let xy = content.split(": ").nth(1).unwrap();
    let mut xys = xy.trim().split(", ");
    let x = xys.next().unwrap();
    let y = xys.next().unwrap();
    let xs = x.split("=").nth(1).unwrap();
    let xfrom = xs.split("..").nth(0).unwrap().parse::<i32>().unwrap();
    let xto = xs.split("..").nth(1).unwrap().parse::<i32>().unwrap();
    let ys = y.split("=").nth(1).unwrap();
    let yfrom = ys.split("..").nth(0).unwrap().parse::<i32>().unwrap();
    let yto = ys.split("..").nth(1).unwrap().parse::<i32>().unwrap();

    println!("{} {} {} {}", xfrom, xto, yfrom, yto);

    let mut maxy = 0;
    let mut cnt = 0;
    for vel_x in -500..500 {
        for vel_y in -500..500 {
            let (max, ok) = simulate(vel_x, vel_y, (xfrom, xto), (yfrom, yto));
            if ok {
                if max > maxy {
                    maxy = max;
                    // println!("({}, {}) -> {}", vel_x, vel_y, maxy);
                }
                cnt += 1;
                // println!("{} {}", vel_x, vel_y);
            }

        }
    }
    println!("{}", maxy);
    println!("{}", cnt);
}

fn simulate(mut vel_x: i32, mut vel_y: i32, xrange: (i32, i32), yrange: (i32, i32)) -> (i32, bool) {
    let mut maxy = 0;
    let mut pos = (0, 0);
    let mut ok = false;
    loop {
        pos = (pos.0 + vel_x, pos.1 + vel_y);
        if pos.1 > maxy {
            maxy = pos.1;
        }
        if vel_x > 0 {
            vel_x -= 1;
        } else if vel_x < 0 {
            vel_x += 1;
        }
        vel_y -= 1;
        if pos.0 >= xrange.0 && pos.0 <= xrange.1 && pos.1 >= yrange.0 && pos.1 <= yrange.1 {
            ok = true;
            break;
        }
        if vel_x == 0 {
            if pos.0 < xrange.0 || pos.0 > xrange.1 {
                break;
            }
            if vel_y < 0 && pos.1 < yrange.0 {
                break;
            }
        }
    }
    (maxy, ok)
}

