use std::{collections::{HashSet, HashMap}, cmp};

fn print_grid(game: &Game) {
    for l in game.grid.iter() {
        println!("{}", l.iter().collect::<String>());
    }
}

fn can_go(
    grid: &Vec<Vec<char>>,
    from: (usize, usize),
    to: (usize, usize),
) -> bool {
    if from == to {
        return true;
    }
    
    if from.0 == 1 { // hallway to room
        for j in cmp::min(from.1, to.1)..=cmp::max(from.1, to.1) {
            if j != from.1 && grid[1][j] != '.' {
                return false;
            }
        }
        for i in 1..=to.0 {
            if grid[i][to.1] != '.' {
                return false;
            }
        }
    } else { // room to hallway
        for i in 1..from.0 {
            if grid[i][from.1] != '.' {
                return false;
            }
        }
        for j in cmp::min(from.1, to.1)..=cmp::max(from.1, to.1) {
            if grid[1][j] != '.' {
                return false;
            }
        }
    }
    return true;
}

macro_rules! add_state {
    ($possible:expr, $game:expr, $poss:expr, $game_room:expr, $cost_coeff:expr, $field:ident, $letter:expr) => {
        for c @ (i, j) in $poss.iter() {
            let room_pos = $game_room;
            if *i != 1 { // room to hallway
                let mut moveable = false;
                if room_pos.contains(c) {
                    for k in i+1..$game.grid.len()-1 {
                        if $game.grid[k][*j] != $letter {
                            moveable = true;
                            break;
                        }
                    }
                } else {
                    moveable = true;
                }
                if moveable {
                    for to @ (ti, tj) in $game.stop_hallways() {
                        if can_go(&$game.grid, *c, to) {
                            let mut newgame = $game.clone();
                            newgame.grid[*i][*j] = '.';
                            newgame.grid[ti][tj] = $letter;
                            newgame.$field.remove(c);
                            newgame.$field.insert(to);
                            newgame.cost += $cost_coeff * ((ti as i32 - *i as i32).abs() + (tj as i32 - *j as i32).abs()) as u64;
                            $possible.push(newgame);
                        }
                    }
                }
            } else { // hallway to room
                let mut target = None;
                for candidate@(ci, cj) in room_pos.iter().rev() {
                    if $game.grid[*ci][*cj] == '.' {
                        target = Some(candidate);
                        break;
                    } else if $game.grid[*ci][*cj] == $letter {
                        continue;
                    } else {
                        break;
                    }
                }
                if let Some(to@(ti, tj)) = target {
                    if can_go(&$game.grid, *c, *to) {
                        let mut newgame = $game.clone();
                        newgame.grid[*i][*j] = '.';
                        newgame.grid[*ti][*tj] = $letter;
                        newgame.$field.remove(c);
                        newgame.$field.insert(*to);
                        newgame.cost += $cost_coeff * ((*ti as i32 - *i as i32).abs() + (*tj as i32 - *j as i32).abs()) as u64;
                        $possible.push(newgame);
                    }
                }
            }
        }
    }
}

fn possible_state(game: &Game) -> Vec<Game> {
    let mut possible = vec![];
    // A
    add_state!(possible, game, game.amber, game.room_amber(), 1, amber, 'A');
    // B
    add_state!(possible, game, game.bronze, game.room_bronze(), 10, bronze, 'B');
    // C
    add_state!(possible, game, game.copper, game.room_copper(), 100, copper, 'C');
    // D
    add_state!(possible, game, game.desert, game.room_desert(), 1000, desert, 'D');

    possible
}

#[derive(Clone, Debug)]
struct Game {
    grid: Vec<Vec<char>>,
    cost: u64,
    amber: HashSet<(usize, usize)>,
    bronze: HashSet<(usize, usize)>,
    copper: HashSet<(usize, usize)>,
    desert: HashSet<(usize, usize)>,
}

impl Game {
    fn room_amber(&self) -> Vec<(usize, usize)> {
        // vec![(2, 3), (3, 3)]
        vec![(2, 3), (3, 3), (4, 3), (5, 3)]
    }
    fn room_bronze(&self) -> Vec<(usize, usize)> {
        // vec![(2, 5), (3, 5)]
        vec![(2, 5), (3, 5), (4, 5), (5, 5)]
    }
    fn room_copper(&self) -> Vec<(usize, usize)> {
        // vec![(2, 7), (3, 7)]
        vec![(2, 7), (3, 7), (4, 7), (5, 7)]
    }
    fn room_desert(&self) -> Vec<(usize, usize)> {
        // vec![(2, 9), (3, 9)]
        vec![(2, 9), (3, 9), (4, 9), (5, 9)]
    }
    fn stop_hallways(&self) -> HashSet<(usize, usize)> {
        HashSet::from([(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)])
            .into_iter()
            .filter(|(i, j)| self.grid[*i][*j] == '.')
            .collect()
    }

    fn win(&self) -> bool {
        // self.amber == HashSet::from([(2, 3), (3, 3)])
        //     && self.bronze == HashSet::from([(2, 5), (3, 5)])
        //     && self.copper == HashSet::from([(2, 7), (3, 7)])
        //     && self.desert == HashSet::from([(2, 9), (3, 9)])
        self.amber == HashSet::from([(2, 3), (3, 3), (4, 3), (5, 3)])
            && self.bronze == HashSet::from([(2, 5), (3, 5), (4, 5), (5, 5)])
            && self.copper == HashSet::from([(2, 7), (3, 7), (4, 7), (5, 7)])
            && self.desert == HashSet::from([(2, 9), (3, 9), (4, 9), (5, 9)])
    }
}

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut grid = vec![];
    for l in content.lines() {
        grid.push(l.chars().collect::<Vec<_>>());
    }
    let mut game = Game {
        grid,
        cost: 0,
        amber: HashSet::new(),
        bronze: HashSet::new(),
        copper: HashSet::new(),
        desert: HashSet::new(),
    };
    for (i, l) in game.grid.iter().enumerate() {
        for (j, ch) in l.iter().enumerate() {
            match *ch {
                'A' => {
                    game.amber.insert((i, j));
                }
                'B' => {
                    game.bronze.insert((i, j));
                }
                'C' => {
                    game.copper.insert((i, j));
                }
                'D' => {
                    game.desert.insert((i, j));
                }
                _ => {}
            }
        }
    }

    dfs(&game);
}

fn dfs(game: &Game) {
    let mut q = vec![];
    let mut visited = HashMap::new();
    visited.insert(game.grid.clone(), game.cost);
    q.push(game.clone());
    let mut min = 999999999;
    if game.win() {
        println!("already win: {}", game.cost);
    }
    while !q.is_empty() {
        let cur = q.pop().unwrap();
        // println!("{:?}", cur);
        // print_grid(&cur);
        // println!("------------------");
        for possible in possible_state(&cur) {

            if possible.win() {
                if possible.cost < min {
                    println!("found: {}", possible.cost);
                    min = possible.cost;
                }
            } else if !visited.contains_key(&possible.grid) {
                visited.insert(possible.grid.clone(), possible.cost);
                q.push(possible);
            } else if visited[&possible.grid] > possible.cost {
                visited.insert(possible.grid.clone(), possible.cost);
                q.push(possible);
            }
        }
        // break;
    }
    println!("{}", min);
}
