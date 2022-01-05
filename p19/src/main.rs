use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI, rc::Rc,
};

use ndarray::{arr2, Array, Array1, Array2};
use ndarray::prelude::*;

fn generate_rotation_matrix() -> HashSet<ndarray::Array2<i32>> {
    let mut t = HashSet::new();
    for deg in [0., 90., 180., 270.] {
        let rad = deg / 180. * PI;
        let x = arr2(&[
            [1, 0, 0],
            [0, rad.cos() as i32, -rad.sin() as i32],
            [0, rad.sin() as i32, rad.cos() as i32],
        ]);
        let y = arr2(&[
            [rad.cos() as i32, 0, rad.sin() as i32],
            [0, 1, 0],
            [-rad.sin() as i32, 0, rad.cos() as i32],
        ]);
        let z = arr2(&[
            [rad.cos() as i32, -rad.sin() as i32, 0],
            [rad.sin() as i32, rad.cos() as i32, 0],
            [0, 0, 1],
        ]);
        t.insert(x);
        t.insert(y);
        t.insert(z);
    }
    let mut all_matrix = HashSet::new();
    for m in t.iter() {
        for n in t.iter() {
            let k = m.dot(n);
            all_matrix.insert(m.clone());
            all_matrix.insert(n.clone());
            all_matrix.insert(k);
        }
    }
    all_matrix
}

fn main() {
    let all_matrix = generate_rotation_matrix();
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut state = 0;
    let mut vecs = vec![];
    let mut beacons = vec![];
    let mut n = 0;
    for l in content.lines() {
        if state == 0 {
            if l.starts_with("--- scanner ") {
                state = 1;
            }
        } else if state == 1 {
            if l != "" {
                l.split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .for_each(|x| {
                        vecs.push(x);
                    });
                n += 1;
            } else {
                state = 0;
                beacons.push(Array::from_shape_vec((n, 3), vecs).unwrap());
                n = 0;
                vecs = vec![];
            }
        }
    }
    beacons.push(Array::from_shape_vec((n, 3), vecs).unwrap());
    analyze(&beacons, &all_matrix);
}

fn analyze(beacons: &Vec<Array2<i32>>, rot_matrix: &HashSet<Array2<i32>>) {
    let mut result = HashMap::new();
    for i in 0..beacons.len() {
        for j in i + 1..beacons.len() {
            println!("{}, {}", i, j);
            match max_possible_overlap(&beacons[i], &beacons[j], rot_matrix) {
                Some((offset, rot)) => {
                    let v = result.entry(j).or_insert(vec![]);
                    v.push((i, offset, rot));
                    if let Some((inv_offset, inv_rot)) = max_possible_overlap(&beacons[j], &beacons[i], rot_matrix) {
                        let v = result.entry(i).or_insert(vec![]);
                        v.push((j, inv_offset, inv_rot));
                    } else {
                        panic!("can't find inverse relation");
                    }
                },
                _ => {},
            }
        }
    }

    let mut processed: HashMap<usize, (Array1<i32>, Rc<dyn Fn(&Array1<i32>)->Array1<i32>>)> = HashMap::new();
    let mut unprocessed = HashSet::new();
    let mut all_coords = HashSet::new();
    processed.insert(0, (arr1(&[0, 0, 0]), Rc::new(|x| x.clone())));
    for coord in beacons[0].rows() {
        all_coords.insert((coord[0], coord[1], coord[2]));
    }

    for i in 1..beacons.len() {
        unprocessed.insert(i);
    }

    while !unprocessed.is_empty() {
        let t = unprocessed.clone();
        for i in t.iter() {
            for (idx, offset, rot) in result[i].iter() {
                if processed.contains_key(&idx) {
                    println!("determining {} from {}", i, idx);
                    let (_, f) = processed[idx].clone();
                    let real_pos = f(&offset);
                    unprocessed.remove(i);

                    let coords = beacons[*i].clone();
                    let tt = coords.dot(rot) + offset;
                    for r in tt.rows() {
                        let rr = f(&r.to_owned());
                        all_coords.insert((rr[0], rr[1], rr[2]));
                    }
                    println!("len: {} {}", i, all_coords.len());
                    processed.insert(*i, (real_pos.clone(), Rc::new(move |x| {
                        let t = x.dot(rot) + offset;
                        f(&t)
                    })));
                }
            }
        }
    }
    // println!("{:?}", all_coords);
    for (k, v) in processed.clone() {
        println!("{} {:?}", k, v.0);
    }
    println!("{}", all_coords.len());

    let mut max_dist = 0;
    for i in 0..beacons.len() {
        for j in i+1..beacons.len() {
            let coord1 = processed[&i].0.clone();
            let coord2 = processed[&j].0.clone();
            let diff = coord1 - coord2;
            let dist = diff.iter().map(|x| x.abs()).sum();
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }
    println!("{}", max_dist);
}

fn max_possible_overlap(
    coords1: &Array2<i32>,
    coords2: &Array2<i32>,
    rots: &HashSet<Array2<i32>>,
) -> Option<(Array1<i32>, Array2<i32>)> {
    for rot in rots {
        let mut h = HashMap::new();
        for coord1 in coords1.rows() {
            for coord2 in coords2.rows() {
                let n = coord2.dot(rot);
                let c1 = coord1.to_owned();
                let c2 = n.clone();
                *h.entry(c1-c2).or_insert(0) += 1;
            }
        }
        for (k, v) in h {
            if v >= 12 {
                return Some((k, rot.clone()));
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use ndarray::array;
    use ndarray_linalg::solve::Inverse;

    #[test]
    fn test_inv_rot() {
        let v = array![[1., 2., 3.]];
        let rot = array![
               [-1., 0., 0.],
               [0., 1., 0.],
               [0., 0., -1.]];
        let r = v.dot(&rot);
        println!("{}", r);
        let rot_v = rot.inv().unwrap();
        let rv = r.dot(&rot_v);
        assert_eq!(rv, v);
    }
}
