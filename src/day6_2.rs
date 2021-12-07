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

    const N: usize = 256;

    let mut cache: Vec<usize> = vec![2, 2, 2, 2, 2, 2, 2, 3, 3];
    for i in 9..N {
        cache.push(cache[i - 7] + cache[i - 9]);
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
