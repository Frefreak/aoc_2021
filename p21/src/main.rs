use std::collections::HashMap;

fn main() {
    let p1 = std::env::args().nth(1).unwrap().parse().unwrap();
    let p2 = std::env::args().nth(2).unwrap().parse().unwrap();

    solve1(p1, p2);
    solve2(p1, p2);
}

trait Dice {
    fn cast(&mut self) -> u32;
    fn rolled(&self) -> u32;

}

struct DeterministicDice {
    rolled: u32,
    next: u32,
}

impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice {
            rolled: 0,
            next: 1,
        }
    }
}

impl Dice for DeterministicDice {
    fn cast(&mut self) -> u32 {
        let o = self.next;
        self.next = self.next + 1;
        if self.next == 101 {
            self.next = 1;
        }
        self.rolled += 1;
        o
    }

    fn rolled(&self) -> u32 {
        self.rolled
    }
}

fn solve1(mut p1: u32, mut p2: u32) {
    let mut d = DeterministicDice::new();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut cur = 0;
    while s1 < 1000 && s2 < 1000 {
        // println!("{} {}", s1, s2);
        let step = d.cast() + d.cast() + d.cast();
        if cur == 0 {
            p1 = (p1 + step - 1) % 10 + 1;
            s1 += p1;
            cur = 1;
        } else {
            p2 = (p2 + step - 1) % 10 + 1;
            s2 += p2;
            cur = 0;
        }
    }
    let loss = if s1 >= 1000 {
        s2
    } else {
        s1
    };
    println!("{} x {} = {}", loss, d.rolled(), loss * d.rolled());
}

fn solve2(p1: u32, p2: u32) {
    let mut cur = 0;
    let mut st: HashMap<(u32, u32, u32, u32), u64> = HashMap::new();
    st.insert((0, 0, p1, p2), 1);
    let mut all_st = vec![];
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                all_st.push(i + j + k);
            }
        }
    }
    let mut w1 = 0;
    let mut w2 = 0;
    while !st.is_empty() {
        let mut new_st: HashMap<(u32, u32, u32, u32), u64> = HashMap::new();
        if cur == 0 {
            for ((s1, s2, p1, p2), v) in &st {
                for st in &all_st {
                    let tp1 = (p1 + st - 1) % 10 + 1;
                    let ts1 = s1 + tp1;
                    if ts1 >= 21 {
                        w1 += v;
                    } else {
                        let ent = new_st.entry((ts1, *s2, tp1, *p2)).or_insert(0);
                        *ent = *ent + v;
                    }
                }
            }
            cur = 1;
        } else {
            for ((s1, s2, p1, p2), v) in &st {
                for st in &all_st {
                    let tp2 = (p2 + st - 1) % 10 + 1;
                    let ts2 = s2 + tp2;
                    if ts2 >= 21 {
                        w2 += v;
                    } else {
                        let ent = new_st.entry((*s1, ts2, *p1, tp2)).or_insert(0);
                        *ent = *ent + v;
                    }
                }
            }
            cur = 0;
        }
        st = new_st;
        // println!("{:?}", st);
    }
    if w1 > w2 {
        println!("[{}] {}", w1, w2);
    } else {
        println!("{} [{}]", w1, w2);
    }
}
