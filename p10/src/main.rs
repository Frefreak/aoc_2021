#[derive(Debug)]
enum SyntaxError {
    Incomplete(String),
    Corrupted(char),
}

fn main() {
    let file = std::env::args().nth(1).expect("filename");
    let content = std::fs::read_to_string(file).unwrap();
    let mut points = 0;
    let mut scores = vec![];
    for l in content.lines() {
        let res = analyze(l);
        use SyntaxError::*;
        println!("{:?}", res);
        match res {
            Incomplete(s) => {
                let mut p = 0_u64;
                for ch in s.chars() {
                    p = p * 5 + match ch {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("impossible"),
                    }
                }
                scores.push(p);
            },
            Corrupted(')') => {points += 3},
            Corrupted(']') => {points += 57},
            Corrupted('}') => {points += 1197},
            Corrupted('>') => {points += 25137},
            _ => panic!("invalid result")
        }
    }
    println!("{}", points);
    scores.sort_unstable();
    println!("{:?}", scores[(scores.len()-1)/2]);
}

fn analyze(l: &str) -> SyntaxError {
    let mut stack = vec![];
    for ch in l.chars() {
        if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
            stack.push(ch);
        } else {
            let expected = match stack.pop().unwrap() {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => panic!("not possible"),
            };
            if ch != expected {
                return SyntaxError::Corrupted(ch);
            }
        }
    }
    let mut s = String::new();
    while !stack.is_empty() {
        s.push(match stack.pop().unwrap() {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("not possible"),
        })
    }
    return SyntaxError::Incomplete(s);
}
