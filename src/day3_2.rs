use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut data = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    let mut data2 = data.clone();

    let mut gamma = &Vec::<u8>::new();
    for i in 0..data[0].len() {
        let m = most(i, &data);
        data.retain(|bits| bits[i] == m);
        if data.len() == 1 {
            gamma = data.last().unwrap();
            break;
        }
    }

    let mut epsilon = &Vec::<u8>::new();
    for i in 0..data2[0].len() {
        let m = 1 - most(i, &data2);
        data2.retain(|bits| bits[i] == m);
        if data2.len() == 1 {
            epsilon = data2.last().unwrap();
            break;
        }
    }

    let gamma = gamma
        .iter()
        .fold(0, |result, bit| (result << 1) ^ *bit as u32);
    let epsilon = epsilon
        .iter()
        .fold(0, |result, bit| (result << 1) ^ *bit as u32);

    println!("{}", gamma * epsilon);
}

fn most(i: usize, data: &[Vec<u8>]) -> u8 {
    let mut one_count = 0;
    for bits in data {
        if bits[i] == 1 {
            one_count += 1;
        }
    }

    if one_count >= data.len() / 2 {
        1
    } else {
        0
    }
}
