use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

fn main() {
    let mut file = File::open("input6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut vec = [0; 6];
    for num in input
        .lines()
        .take(1)
        .last()
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<usize>, ParseIntError>>()
        .unwrap()
    {
        vec[num] += 1;
    }

    const N: usize = 80;

    let mut cache = [0; N];
    let mut t = vec![0; 1];
    for c in cache.iter_mut() {
        let mut new_count = 0;
        for num in t.iter_mut() {
            *num -= 1;
            if *num == -1 {
                new_count += 1;
                *num = 6;
            }
        }

        t.resize(t.len() + new_count, 8);
        *c = t.len();
    }

    let mut total = 0;
    for (i, count) in vec.iter().enumerate() {
        if *count == 0 {
            continue;
        }

        total += count * cache[N - i - 1];
    }

    println!("{}", total);
}
