fn main() {
    let mut enhance: Vec<u32> = vec![];
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut lines = content.lines();
    for ch in lines.next().unwrap().chars() {
        if ch == '#' {
            enhance.push(1);
        } else {
            enhance.push(0);
        }
    }
    lines.next();
    let mut img = vec![];
    for l in lines {
        img.push(l.chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<_>>());
    }

    let step = 50;
    let mut expand_img = vec![];
    for _ in 0..step {
        expand_img.push(vec![0; img[0].len() + 2 * step]);
    }
    for l in img.iter() {
        let mut t = vec![0; img[0].len() + 2 * step];
        for (i, ch) in l.iter().enumerate() {
            t[step + i] = *ch;
        }
        expand_img.push(t);
    }
    for _ in 0..step {
        expand_img.push(vec![0; img[0].len() + 2 * step]);
    }
    let mut img = expand_img;
    let mut bg = 0;

    for _i in 0..step {
        img = do_enhance(&img, &enhance, &mut bg);
    }


    let s = img.iter().map(|x| x.iter().sum::<u32>()).sum::<u32>();
    println!("{}", s);

    let mut tga = tgaimage::TGAImage::new(img[0].len() as u16, img.len() as u16);
    for (j, l) in img.iter().enumerate() {
        for (i, ch) in l.iter().enumerate() {
            let p = *ch as u8;
            tga.set(i as u16, j as u16, (p * 255, p * 255, p * 255));
        }
    }
    tga.write_to_file("out.tga").unwrap();
}

fn print_img(img: &Vec<Vec<u32>>) {
    for l in img.iter() {
        println!("{}", l.iter().map(|x| if *x == 0 {'.'} else {'#'}).collect::<String>());
    }
}

fn do_enhance(img: &Vec<Vec<u32>>, enhance: &[u32], bg: &mut u32) -> Vec<Vec<u32>> {
    let mut new_img = img.clone();
    for (j, l) in img.iter().enumerate() {
        for (i, ch) in l.iter().enumerate() {
            let mut s = 0;
            let j = j as i32;
            let i = i as i32;
            for (x, y) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                if j + y >= 0 && j + y < img.len() as i32 && i + x >= 0 && i + x < img[0].len() as i32 {
                    s = s * 2 + img[(j+y) as usize][(i+x) as usize];
                } else {
                    s = s * 2 + *bg;
                }
            }
            new_img[j as usize][i as usize] = enhance[s as usize];
        }
    }
    let mut s = 0;
    for _ in 0..9 {
        s = s * 2 + *bg;
    }
    *bg = enhance[s as usize];
    new_img
}
