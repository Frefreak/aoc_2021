#![feature(derive_default_enum)]

use std::time::SystemTime;
fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let now0 = SystemTime::now();
    let bits: String = content
        .trim()
        .chars()
        .map(|ch| format!("{:04b}", ch.to_digit(16).unwrap()))
        .collect();
    let now1 = SystemTime::now();
    let packet = parse(&mut bits.chars()).unwrap();
    println!("{:#?}", packet);
    let now2 = SystemTime::now();
    println!("parse {:?}", now1.duration_since(now0));
    println!("parse {:?}", now2.duration_since(now1));

    println!("{}", sum_version(&packet));
    println!("{}", eval(&packet));
    let now3 = SystemTime::now();
    println!("sum & eval {:?}", now3.duration_since(now2));
}

#[derive(Debug, Default)]
enum Data {
    #[default]
    Empty,
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Box<Packet>, Box<Packet>),
    LessThan(Box<Packet>, Box<Packet>),
    EqualsTo(Box<Packet>, Box<Packet>),
}

#[derive(Debug, Default)]
struct Packet {
    version: u64,
    data: Data,
}

fn sum_version(p: &Packet) -> u64 {
    p.version
        + match &p.data {
            Data::Literal(_) => 0,
            Data::Sum(v) | Data::Product(v) | Data::Maximum(v) | Data::Minimum(v) => {
                v.iter().map(sum_version).sum()
            }
            Data::GreaterThan(p1, p2) | Data::LessThan(p1, p2) | Data::EqualsTo(p1, p2) => {
                sum_version(p1) + sum_version(p2)
            }
            _ => todo!(),
        }
}

fn eval(p: &Packet) -> u64 {
    match &p.data {
        Data::Literal(v) => *v,
        Data::Sum(vec) => vec.iter().map(eval).sum(),
        Data::Product(vec) => vec.iter().map(eval).product(),
        Data::Minimum(vec) => vec.iter().map(eval).min().unwrap(),
        Data::Maximum(vec) => vec.iter().map(eval).max().unwrap(),
        Data::GreaterThan(p1, p2) => {
            if eval(p1) > eval(p2) {
                1
            } else {
                0
            }
        }
        Data::LessThan(p1, p2) => {
            if eval(p1) < eval(p2) {
                1
            } else {
                0
            }
        }
        Data::EqualsTo(p1, p2) => {
            if eval(p1) == eval(p2) {
                1
            } else {
                0
            }
        }
        _ => todo!(),
    }
}

fn parse<T: Iterator<Item = char>>(bits: &mut T) -> Option<Packet> {
    let version = u64::from_str_radix(&bits.take(3).collect::<String>(), 2);
    if version.is_err() {
        return None;
    }
    let version = version.unwrap();
    let ty = u64::from_str_radix(&bits.take(3).collect::<String>(), 2).unwrap();
    let data = match ty {
        4 => {
            let mut s = String::new();
            loop {
                let ind = bits.take(1).collect::<String>();
                let n = bits.take(4).collect::<String>();
                s += &n;
                if ind == "0" {
                    break;
                }
            }
            Data::Literal(u64::from_str_radix(&s, 2).unwrap())
        }
        _ => {
            let b = bits.take(1).next().unwrap();
            let mut arr = match b {
                '0' => {
                    let length =
                        u64::from_str_radix(&bits.take(15).collect::<String>(), 2).unwrap();
                    let data = bits.take(length as usize).collect::<String>();
                    let mut packets = vec![];
                    let mut chars = data.chars();
                    while let Some(p) = parse(&mut chars) {
                        packets.push(p);
                    }
                    packets
                }
                '1' => {
                    let size = u64::from_str_radix(&bits.take(11).collect::<String>(), 2).unwrap();
                    let mut packets = vec![];
                    for _i in 0..size {
                        packets.push(parse(bits).unwrap());
                    }
                    packets
                }
                _ => {
                    panic!("impossible")
                }
            };
            match ty {
                0 => Data::Sum(arr),
                1 => Data::Product(arr),
                2 => Data::Minimum(arr),
                3 => Data::Maximum(arr),
                5 => {
                    assert_eq!(2, arr.len());
                    Data::GreaterThan(Box::new(arr.remove(0)), Box::new(arr.remove(0)))
                }
                6 => {
                    assert_eq!(2, arr.len());
                    Data::LessThan(Box::new(arr.remove(0)), Box::new(arr.remove(0)))
                }
                7 => {
                    assert_eq!(2, arr.len());
                    Data::EqualsTo(Box::new(arr.remove(0)), Box::new(arr.remove(0)))
                }
                _ => panic!("impossible"),
            }
        }
    };
    Some(Packet { version, data })
}
