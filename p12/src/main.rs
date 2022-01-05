use std::collections::{HashMap, HashSet};

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut routes: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in content.lines() {
        let mut segs = l.split("-");
        let from = segs.next().unwrap();
        let to = segs.next().unwrap();

        if let Some(entry) = routes.get_mut(&from) {
            entry.push(to);
        } else {
            routes.insert(from, vec![to]);
        }

        if let Some(entry) = routes.get_mut(&to) {
            entry.push(from);
        } else {
            routes.insert(to, vec![from]);
        }
    }
    let mut visited: HashSet<&str> = HashSet::new();
    let nroutes = dfs("start", &routes, &mut visited);
    println!("{}", nroutes);
    let mut visited: HashMap<&str, u32> = HashMap::new();
    let nroutes = dfs2("start", &routes, &mut visited, false);
    println!("{}", nroutes);
}

fn dfs<'a>(
    start: &'a str,
    routes: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
) -> u32 {
    // println!("enter {}", start);
    if start == "end" {
        // println!("found");
        return 1;
    }
    visited.insert(start);
    let mut nroute = 0;
    if let Some(children) = routes.get(start) {
        for child in children {
            if child.chars().all(|x| x.is_uppercase()) {
                nroute += dfs(child, routes, visited);
            } else {
                if !visited.contains(child) {
                    nroute += dfs(child, routes, visited);
                }
            }
        }
    }
    visited.remove(start);
    // println!("leave {}", start);
    nroute
}

fn dfs2<'a>(
    start: &'a str,
    routes: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashMap<&'a str, u32>,
    restricted: bool,
) -> u32 {
    // println!("enter {} {:?}", start, visited);
    if start == "end" {
        // println!("found");
        return 1;
    }
    if let Some(entry) = visited.get_mut(start) {
        *entry += 1;
    } else {
        visited.insert(start, 1);
    }
    let mut nroute = 0;
    if let Some(children) = routes.get(start) {
        for child in children {
            if child.chars().all(|x| x.is_uppercase()) {
                nroute += dfs2(child, routes, visited, restricted);
            } else {
                if *child == "start" {
                    continue;
                }
                if let Some(k) = visited.get(child) {
                    if *k == 0 {
                        nroute += dfs2(child, routes, visited, restricted);
                    } else if *k == 1 {
                        if !restricted {
                            nroute += dfs2(child, routes, visited, true);
                        }
                    }
                } else {
                    nroute += dfs2(child, routes, visited, restricted);
                }
            }
        }
    }
    let x = visited.get_mut(start).unwrap();
    *x -= 1;
    // println!("leave {}", start);
    nroute
}
