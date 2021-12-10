use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input10.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut score = 0u64;
    for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                }
                ')' | ']' | '}' | '>' => {
                    let l = stack.pop();
                    if l.is_none() || get_reverse(l.unwrap()) != c {
                        score += calculate_score(c);
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    println!("{}", score);
}

fn get_reverse(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn calculate_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}
