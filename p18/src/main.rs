use std::{str::FromStr, fmt::Display, ops::Add};

#[macro_use]
mod macros;

#[derive(PartialEq, Debug, Clone)]
enum Snailfish {
    Pair(Box<Snailfish>, Box<Snailfish>),
    Num(u32),
}

impl Snailfish {
    fn get_num(&self) -> Option<u32> {
        match self {
            Snailfish::Num(n) => Some(*n),
            _ => None,
        }
    }
    fn is_num(&self) -> bool {
        match self {
            Snailfish::Num(_) => true,
            _ => false,
        }
    }
    #[allow(dead_code)]
    fn is_pair(&self) -> bool {
        !self.is_num()
    }

    fn reduce(&mut self) {
        loop {
            let b = self.explode();
            if b {
                continue;
            }

            let b = self.split();
            if b {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self) -> bool {
        self.real_explode(0, &mut None, &mut None)
    }
    fn split(&mut self) -> bool {
        self.real_split()
    }

    fn real_explode(&mut self, level: usize, left: &mut Option<&mut Snailfish>, right: &mut Option<&mut Snailfish>) -> bool {
        match self {
            Snailfish::Num(_) => {
                false
            },
            Snailfish::Pair(p1, p2) => {
                if level == 4 && p1.is_num() && p2.is_num() {
                    let ln = p1.get_num().unwrap();
                    let rn = p2.get_num().unwrap();
                    if let Some(t) = left.take() {
                        // **t = Snailfish::Num(t.get_num().unwrap() + ln);
                        *t = Snailfish::Num(t.get_num().unwrap() + ln);
                    }
                    if let Some(t) = right.take() {
                        *t = Snailfish::Num(t.get_num().unwrap() + rn);
                    }
                    *self = Snailfish::Num(0);
                    true 
                } else {
                    let mut new_right = p2.get_left_num();
                    let b = if new_right.is_some() {
                        p1.real_explode(level + 1, left, &mut new_right)
                    } else {
                        p1.real_explode(level + 1, left, right)
                    };

                    if b {
                        return b
                    }
                    let mut new_left = p1.get_right_num();
                    let b = if new_left.is_some() {
                        p2.real_explode(level + 1, &mut new_left, right)
                    } else {
                        p2.real_explode(level + 1, left, right)
                    };
                    return b;
                }
            }
        }
    }

    fn get_left_num(&mut self) -> Option<&mut Snailfish> {
        match self {
            Snailfish::Num(_) => Some(self),
            Snailfish::Pair(p1, _) => {
                p1.get_left_num()
            }
        }
    }
    fn get_right_num(&mut self) -> Option<&mut Snailfish> {
        match self {
            Snailfish::Num(_) => Some(self),
            Snailfish::Pair(_, p2) => {
                p2.get_right_num()
            }
        }
    }

    fn real_split(&mut self) -> bool {
        match self {
            Snailfish::Pair(p1, p2) => {
                p1.real_split() || p2.real_split()
            },
            Snailfish::Num(n) if *n >= 10 => {
                let first = *n / 2;
                let second = *n - first;
                *self = SP!(SN!(first), SN!(second));
                true
            },
            _ => {
                false
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Snailfish::Num(n) => *n,
            Snailfish::Pair(p1, p2) => {
                3 * p1.magnitude() + 2 * p2.magnitude()
            }
        }
    }
}

impl FromStr for Snailfish {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect();
        conv(&chars, &mut 0)
    }
}

impl Add for Snailfish {
    type Output = Snailfish;

    fn add(self, rhs: Self) -> Self::Output {
        let mut n = SP!(self, rhs);
        n.reduce();
        n
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Snailfish::Num(n) => f.write_fmt(format_args!("{}", n)),
            Snailfish::Pair(p1, p2) => {
                f.write_str("[")?;
                f.write_fmt(format_args!("{}", *p1))?;
                f.write_str(",")?;
                f.write_fmt(format_args!("{}", *p2))?;
                f.write_str("]")?;
                Ok(())
            }
        }
    }
}

fn conv(chars: &Vec<char>, pos: &mut usize) -> Result<Snailfish, std::io::Error> {
    if chars[*pos] == '[' {
        *pos += 1;
        let p1 = conv(&chars, pos)?;
        assert_eq!(*&chars[*pos], ',');
        *pos += 1;
        let p2 = conv(&chars, pos)?;
        assert_eq!(*&chars[*pos], ']');
        *pos += 1;
        Ok(Snailfish::Pair(Box::new(p1), Box::new(p2)))
    } else {
        let mut n = chars[*pos].to_digit(10).unwrap();
        *pos += 1;
        while *pos < chars.len() && chars[*pos] <= '9' && chars[*pos] >= '0' {
            n = n * 10 + chars[*pos].to_digit(10).unwrap();
            *pos += 1;
        }
        Ok(Snailfish::Num(n))
    }
}

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let nums: Vec<_> = content.lines().map(|x| x.parse::<Snailfish>().unwrap()).collect();
    let r = nums.into_iter().reduce(|x, y| x + y).unwrap();
    println!("{}", r.magnitude());

    let nums: Vec<_> = content.lines().map(|x| x.parse::<Snailfish>().unwrap()).collect();
    let mut max_m = 0;
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            let r1 = nums[i].clone() + nums[j].clone();
            let r2 = nums[j].clone() + nums[i].clone();
            if r1.magnitude() > max_m {
                max_m = r1.magnitude();
            } else if r2.magnitude() > max_m {
                max_m = r2.magnitude()
            }
        }
    }
    println!("{}", max_m);
}

#[allow(unused_macros)]
macro_rules! SP {
    ($p1:expr, $p2:expr) => {
        Snailfish::Pair(Box::new($p1), Box::new($p2))
    };
}

#[allow(unused_macros)]
macro_rules! SN {
    ($p1:expr) => {
        Snailfish::Num($p1)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_str() {
        let e1 = "1".parse::<Snailfish>().unwrap();
        assert_eq!(e1, SN!(1));
        let e2 = "[1,2]".parse::<Snailfish>().unwrap();
        assert_eq!(e2, SP!(SN!(1), SN!(2)));
        let e3 = "[[1,2],3]".parse::<Snailfish>().unwrap();
        assert_eq!(e3, SP!(e2, SN!(3)));
        let e4 = "[9,[8,7]]".parse::<Snailfish>().unwrap();
        assert_eq!(e4, SP!(SN!(9), SP!(SN!(8), SN!(7))));
        let e5 = "[[1,9],[8,5]]".parse::<Snailfish>().unwrap();
        assert_eq!(e5, SP!(SP!(SN!(1), SN!(9)), SP!(SN!(8), SN!(5))));
        let e6 = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".parse::<Snailfish>().unwrap();
        let sub1 = SP!(SP!(SN!(1), SN!(2)), SP!(SN!(3), SN!(4)));
        let sub2 = SP!(SP!(SN!(5), SN!(6)), SP!(SN!(7), SN!(8)));
        assert_eq!(e6, SP!(SP!(sub1, sub2), SN!(9)));
    }

    #[test]
    fn test_display() {
        let expr1 = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]";
        let expr2 = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let e1 = expr1.parse::<Snailfish>().unwrap();
        assert_eq!(expr1, format!("{}", e1));
        let e2 = expr2.parse::<Snailfish>().unwrap();
        assert_eq!(expr2, format!("{}", e2));
    }

    #[test]
    fn test_explode() {
        let expr1 = "[[[[[9,8],1],2],3],4]";
        let mut e1 = expr1.parse::<Snailfish>().unwrap();
        let ret = e1.explode();
        assert_eq!("[[[[0,9],2],3],4]", format!("{}", e1));
        assert_eq!(ret, true);

        let expr2 = "[7,[6,[5,[4,[3,2]]]]]";
        let mut e2 = expr2.parse::<Snailfish>().unwrap();
        let ret = e2.explode();
        assert_eq!("[7,[6,[5,[7,0]]]]", format!("{}", e2));
        assert_eq!(ret, true);

        let expr3 = "[[6,[5,[4,[3,2]]]],1]";
        let mut e3 = expr3.parse::<Snailfish>().unwrap();
        let ret = e3.explode();
        assert_eq!("[[6,[5,[7,0]]],3]", format!("{}", e3));
        assert_eq!(ret, true);

        let expr4 = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let mut e4 = expr4.parse::<Snailfish>().unwrap();
        let ret = e4.explode();
        assert_eq!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", format!("{}", e4));
        assert_eq!(ret, true);

        let expr5 = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let mut e5 = expr5.parse::<Snailfish>().unwrap();
        let ret = e5.explode();
        assert_eq!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", format!("{}", e5));
        assert_eq!(ret, true);

        let expr6 = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let mut e6 = expr6.parse::<Snailfish>().unwrap();
        let ret = e6.explode();
        assert_eq!("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", format!("{}", e6));
        assert_eq!(ret, true);
        let ret = e6.explode();
        assert_eq!("[[[[0,7],4],[15,[0,13]]],[1,1]]", format!("{}", e6));
        assert_eq!(ret, true);
        let ret = e6.explode();
        assert_eq!(ret, false);
    }

    #[test]
    fn test_split() {
        let expr1 = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let mut e1 = expr1.parse::<Snailfish>().unwrap();
        let ret = e1.split();
        assert_eq!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", format!("{}", e1));
        assert_eq!(ret, true);

        let ret = e1.split();
        assert_eq!("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", format!("{}", e1));
        assert_eq!(ret, true);
    }

    #[test]
    fn test_reduce() {
        let expr1 = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let mut e1 = expr1.parse::<Snailfish>().unwrap();
        e1.reduce();
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", format!("{}", e1));
    }

    #[test]
    fn test_add() {
        let expr1: Snailfish = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse().unwrap();
        let expr2: Snailfish = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse().unwrap();
        let expr3: Snailfish = "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".parse().unwrap();
        let expr4: Snailfish = "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]".parse().unwrap();

        let r1 = expr1 + expr2;
        assert_eq!("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]", format!("{}", r1));
        let r2 = r1 + expr3;
        assert_eq!("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]", format!("{}", r2));
        let r3 = r2 + expr4;
        assert_eq!("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]", format!("{}", r3));

        let expr5 = "[7,[5,[[3,8],[1,4]]]]".parse().unwrap();
        let expr6 = "[[2,[2,2]],[8,[8,1]]]".parse().unwrap();
        let expr7 = "[2,9]".parse().unwrap();
        let expr8 = "[1,[[[9,3],9],[[9,0],[0,7]]]]".parse().unwrap();
        let expr9 = "[[[5,[7,4]],7],1]".parse().unwrap();
        let expr10 = "[[[[4,2],2],6],[8,7]]".parse().unwrap();
        let r = r3 + expr5 + expr6 + expr7 + expr8 + expr9 + expr10;
        assert_eq!("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", format!("{}", r));
    }

    #[test]
    fn test_magnitude() {
        let expr: Snailfish = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap();
        assert_eq!(4140, expr.magnitude());
    }
}
