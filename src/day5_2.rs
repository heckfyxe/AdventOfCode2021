use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut matrix = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let mut points = parse(line);
        if points[0] != points[2]
            && points[1] != points[3]
            && (points[0] - points[2]).abs() != (points[1] - points[3]).abs()
        {
            continue;
        }

        if points[0] == points[2] {
            if points[1] > points[3] {
                points.swap(1, 3);
            }

            for i in points[1]..=points[3] {
                matrix[points[0] as usize][i as usize] += 1;
            }

            continue;
        }

        if points[1] == points[3] {
            if points[0] > points[2] {
                points.swap(0, 2);
            }

            for i in points[0]..=points[2] {
                matrix[i as usize][points[1] as usize] += 1
            }

            continue;
        }

        let dx = (points[2] - points[0]) / (points[2] - points[0]).abs();
        let dy = (points[3] - points[1]) / (points[3] - points[1]).abs();

        for i in 0..=(points[2] - points[0]).abs() {
            matrix[(points[0] + i * dx) as usize][(points[1] + i * dy) as usize] += 1;
        }
    }

    let mut count = 0;
    for vec in matrix {
        for v in vec {
            if v >= 2 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn parse(line: &str) -> [i32; 4] {
    let mut vec = [0; 4];
    for (i, point) in line.split(" -> ").enumerate() {
        for (j, coor) in point.split(',').enumerate() {
            vec[i * 2 + j] = coor.parse().unwrap();
        }
    }
    vec
}
