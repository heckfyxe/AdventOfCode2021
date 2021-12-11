use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input11.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut matrix = vec![];
    for line in input.lines() {
        matrix.push(line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>());
    }

    let mut count = 0usize;
    for _ in 1..=100 {
        for row in matrix.iter_mut() {
            for num in row.iter_mut() {
                *num += 1;
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                count += rec(&mut matrix, i, j);
            }
        }
    }

    println!("{}", count);
}

fn rec(matrix: &mut [Vec<u8>], i: usize, j: usize) -> usize {
    if matrix[i][j] < 10 {
        return 0;
    }

    matrix[i][j] = 0;

    let mut count = 1;
    for i2 in (i as isize - 1)..=(i as isize + 1) {
        for j2 in (j as isize - 1)..=(j as isize + 1) {
            if i2 < 0
                || j2 < 0
                || i2 >= matrix.len() as isize
                || j2 >= matrix[0].len() as isize
                || matrix[i2 as usize][j2 as usize] == 0
            {
                continue;
            }

            matrix[i2 as usize][j2 as usize] += 1;
            count += rec(matrix, i2 as usize, j2 as usize);
        }
    }
    count
}
