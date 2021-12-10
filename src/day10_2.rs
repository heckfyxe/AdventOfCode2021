use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input10.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut scores = Vec::new();
    for line in input.lines() {
        let mut stack = Vec::new();
        let mut invalid = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                }
                ')' | ']' | '}' | '>' => {
                    let l = stack.pop();
                    if l.is_none() || get_reverse(l.unwrap()) != c {
                        invalid = true;
                        break;
                    }
                }
                _ => {}
            }
        }

        if invalid {
            continue;
        }

        let mut score = 0u64;
        for c in stack.iter().rev() {
            score = score * 5 + calculate_score(get_reverse(*c));
        }
        scores.push(score);
    }

    scores.sort_unstable();

    println!("{}", scores[scores.len() / 2]);
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
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}
