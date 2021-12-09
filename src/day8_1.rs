use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input8.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut count = 0u64;
    for line in input.lines() {
        let output: Vec<&str> = line.split('|').last().unwrap().split_whitespace().collect();
        for segments in output {
            if (2..5).contains(&segments.len()) || segments.len() == 7 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
