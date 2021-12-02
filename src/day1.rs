use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let data = str
        .lines()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<i32>, ParseIntError>>()
        .unwrap();

    let mut m = 0xffff;
    let mut count = 0;
    for i in 0..data.len() {
        if data[i] > m {
            count += 1;
        }
        m = data[i];
    }

    println!("{}", count);

    let mut nums = Vec::<i32>::new();
    for i in 0..data.len() - 2 {
        nums.push(data[i] + data[i + 1] + data[i + 2]);
    }

    let mut m = 0xffff;
    let mut count = 0;
    for i in 0..nums.len() {
        if nums[i] > m {
            count += 1;
        }
        m = nums[i];
    }

    println!("{}", count);
}
