use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input2.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in str.lines() {
        let command = line.split(' ').collect::<Vec<&str>>()[0];
        let step = line.split(' ').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        match command {
            "forward" => {
                x += step;
                y += aim * step;
            }
            "down" => aim += step,
            "up" => aim -= step,
            _ => {}
        }
    }

    println!("{}", x * y);
}
