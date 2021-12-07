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

    let median = vec[vec.len() / 2];
    let mut total = 0u64;
    for num in vec {
        total += (num - median).abs() as u64;
    }
    println!("{}", total);
}
