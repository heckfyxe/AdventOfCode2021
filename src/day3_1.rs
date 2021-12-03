use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut bits = [0; 12];
    let mut count = 0;
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                bits[i] += 1;
            }
        }
        count += 1;
    }

    for bit in bits.iter_mut() {
        if *bit > count / 2 {
            *bit = 1;
        } else {
            *bit = 0;
        }
    }

    let gamma = bits.iter().fold(0, |result, &bit| (result << 1) ^ bit);
    let epsilon = bits
        .map(|bit| 1 - bit)
        .iter()
        .fold(0, |result, &bit| (result << 1) ^ bit);

    println!("{}", gamma * epsilon);
}
