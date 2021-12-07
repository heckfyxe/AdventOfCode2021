use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input7.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut vec: Vec<i32> = Vec::new();
    for num in input
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
    {
        vec.push(num);
    }

    vec.sort_unstable();

    let mean = vec.iter().sum::<i32>() / vec.len() as i32;

    let mut total = 0u64;
    for num in vec {
        let n = (num - mean).abs();
        total += (n * (2 + (n - 1)) / 2) as u64;
    }
    println!("{}", total);
}
