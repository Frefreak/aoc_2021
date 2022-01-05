use std::{str::FromStr, num::{IntErrorKind, ParseIntError}};
use hashbrown::HashMap;
use rayon::prelude::*;

#[derive(Clone)]
struct ALU {
    instructions: Vec<Instruction>,
    regs: HashMap<Reg, i64>,
    input: Vec<i64>,
    input_idx: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
enum Reg {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl FromStr for Reg {
    type Err = IntErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Reg::W),
            "x" => Ok(Reg::X),
            "y" => Ok(Reg::Y),
            "z" => Ok(Reg::Z),
            _ => Err(IntErrorKind::InvalidDigit),
        }
    }
}


#[derive(Debug, Clone, Copy)]
enum Operand {
    Reg(Reg),
    Imm(i64),
}

impl FromStr for Operand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => {Ok(Operand::Reg(Reg::W))},
            "x" => {Ok(Operand::Reg(Reg::X))},
            "y" => {Ok(Operand::Reg(Reg::Y))},
            "z" => {Ok(Operand::Reg(Reg::Z))},
            n => {
                match n.parse() {
                    Ok(val) => Ok(Operand::Imm(val)),
                    Err(err) => Err(err),
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Reg),
    Add(Reg, Operand),
    Mul(Reg, Operand),
    Div(Reg, Operand),
    Mod(Reg, Operand),
    Eql(Reg, Operand),
    NoEql(Reg, Operand),
    Assign(Reg, Operand),
}

impl ALU {
    fn new(code: String) -> ALU {
        let instructions = code.lines().map(|x| parse_instruction(x)).collect();
        let regs = [(Reg::W, 0), (Reg::X, 0), (Reg::Y, 0), (Reg::Z, 0)].into_iter().collect();
        let input = vec![];
        let input_idx = 0;
        ALU {
            instructions,
            regs,
            input,
            input_idx,
        }
    }

    fn get_val(&self, c: &Operand) -> i64 {
        match *c {
            Operand::Reg(r) => self.regs[&r],
            Operand::Imm(imm) => imm,
        }
    }

    /// also clear state
    fn set_input(&mut self, mut digit: u64) -> bool {
        let mut v = vec![];
        while digit != 0 {
            let d = (digit % 10) as i64;
            if d == 0 {
                return false;
            }
            v.push(d);
            digit = digit / 10;
        }
        v.reverse();
        self.input = v;
        self.input_idx = 0;
        self.regs = [(Reg::W, 0), (Reg::X, 0), (Reg::Y, 0), (Reg::Z, 0)].into_iter().collect();
        return true;
    }

    fn set_input_arr(&mut self, digits: Vec<i64>) -> bool {
        self.input = digits;
        self.input_idx = 0;
        self.regs = [(Reg::W, 0), (Reg::X, 0), (Reg::Y, 0), (Reg::Z, 0)].into_iter().collect();
        return true;
    }

    fn run(&mut self) {
        for instr in self.instructions.clone().iter() {
            self.eval(instr);
        }
        // for ch in &["w", "x", "y", "z"] {
        //     println!("{}: {}", ch, self.regs[*ch]);
        // }
        // println!("--------------");
    }

    fn eval(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Inp(ch) => {
                self.regs.insert(*ch, self.input[self.input_idx]);
                self.input_idx += 1;
            },
            Instruction::Add(a, b) => {
                let newval = self.regs[a] + self.get_val(b);
                self.regs.insert(*a, newval);
            },
            Instruction::Mul(a, b) => {
                let newval = self.regs[a] * self.get_val(b);
                self.regs.insert(*a, newval);
            },
            Instruction::Div(a, b) => {
                let newval = self.regs[a] / self.get_val(b);
                self.regs.insert(*a, newval);
            },
            Instruction::Mod(a, b) => {
                let newval = self.regs[a] % self.get_val(b);
                self.regs.insert(*a, newval);
            },
            Instruction::Eql(a, b) => {
                let newval = if self.regs[a] == self.get_val(b) {1} else {0};
                self.regs.insert(*a, newval);
            },
            Instruction::NoEql(a, b) => {
                let newval = if self.regs[a] == self.get_val(b) {0} else {1};
                self.regs.insert(*a, newval);
            },
            Instruction::Assign(a, b) => {
                self.regs.insert(*a, self.get_val(b));
            },
        }
    }
}

fn parse_instruction(s: &str) -> Instruction {
    let segs = s.split_whitespace().collect::<Vec<_>>();
    match segs[0] {
        "inp" => Instruction::Inp(segs[1].parse().unwrap()),
        "add" => Instruction::Add(
            segs[1].parse().unwrap(),
            segs[2].parse().unwrap(),
        ),
        "mul" => Instruction::Mul(
            segs[1].parse().unwrap(),
            segs[2].parse().unwrap(),
        ),
        "div" => Instruction::Div(
            segs[1].parse().unwrap(),
            segs[2].parse().unwrap(),
        ),
        "mod" => Instruction::Mod(
            segs[1].parse().unwrap(),
            segs[2].parse().unwrap(),
        ),
        "eql" => Instruction::Eql(
            segs[1].parse().unwrap(),
            segs[2].parse().unwrap(),
        ),
        "noeql" => Instruction::NoEql(
            segs[1].parse().unwrap(),
            segs[2].parse().unwrap(),
        ),
        "assign" => Instruction::Assign(
            segs[1].parse().unwrap(),
            segs[2].parse().unwrap(),
        ),
        _ => panic!("impossible"),
    }
}

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut alu = ALU::new(content);
    let n: usize= 99999999999999;

    // alu.set_input(39924989499969);
    // alu.run();
    // println!("{:?}", alu.regs);
    let mut v = vec![1,2,3,4,5,6,7,8,9];
    // v.reverse();
    let mut sets = vec![];
    for _i in 0..14 {
        sets.push(v.clone());
    }
    for seq in generate_all_seq(vec![vec![]], sets) {
        alu.set_input_arr(seq.clone());
        alu.run();
        if alu.regs[&Reg::Z] == 0 {
            println!("{}", seq.iter().map(|x| x.to_string()).collect::<String>());
            break;
        }
    }

}

fn generate_all_seq(total: Vec<Vec<i64>>, mut sets: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    if sets.len() == 0 {
        return total;
    }
    let first = sets.drain(0..1).collect::<Vec<_>>();
    let mut new_total = vec![];
    for t in total {
        for s in first[0].iter() {
            let mut tt = t.clone();
            tt.push(*s);
            if tt.len() == 4 {
                if tt[2] + 2 - 9 != tt[3] {
                    continue;
                }
            }
            if tt.len() == 5 {
                if tt[1] + 4 - 9 != tt[4] {
                    continue;
                }
            }
            if tt.len() == 8 {
                if tt[6] + 11 - 10 != tt[7] {
                    continue;
                }
            }
            if tt.len() == 10 {
                if tt[8] + 7 - 2 != tt[9] {
                    continue;
                }
            }
            if tt.len() == 12 {
                if tt[10] + 15 - 15 != tt[11] {
                    continue;
                }
            }
            if tt.len() == 13 {
                if tt[5] + 6 - 9 != tt[12] {
                    continue;
                }
            }
            if tt.len() == 14 {
                if tt[0] + 9 - 3 != tt[13] {
                    continue;
                }
            }
            new_total.push(tt);
        }
    }
    generate_all_seq(new_total, sets)
}

#[cfg(test)]
mod test {
    use crate::{ALU, generate_all_seq};

    // #[test]
    fn test_input_equivalent() {
        let content = std::fs::read_to_string("input.txt").unwrap();
        let mut alu1 = ALU::new(content);
        let content = std::fs::read_to_string("input2.txt").unwrap();
        let mut alu2 = ALU::new(content);
        let mut n: u64 = 99999999999999;
        while n > 99999999989999 {
            let r1 = alu1.set_input(n);
            let r2 = alu2.set_input(n);
            assert_eq!(r1, r2);
            if r1 {
                alu1.run();
                alu2.run();
                assert_eq!(alu1.regs, alu2.regs);
            }
            n -= 1;
        }
    }

    fn test_input_equivalent2() {
        let content = std::fs::read_to_string("input2.txt").unwrap();
        let mut alu1 = ALU::new(content);
        let content = std::fs::read_to_string("input3.txt").unwrap();
        let mut alu2 = ALU::new(content);
        let mut n: u64 = 99999999999999;
        while n > 99999999959999 {
            let r1 = alu1.set_input(n);
            let r2 = alu2.set_input(n);
            assert_eq!(r1, r2);
            if r1 {
                alu1.run();
                alu2.run();
                assert_eq!(alu1.regs, alu2.regs);
            }
            n -= 1;
        }
    }

    // #[test]
    fn test_generate_seq() {
        let mut sets = vec![];
        sets.push(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        sets.push(vec![1, 2, 3]);
        let r = generate_all_seq(vec![vec![]], sets);
        println!("{:?}", r);
        assert_eq!(r.len(), 27);
    }

}
