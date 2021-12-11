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

    let mut step = 0usize;
    loop {
        for row in matrix.iter_mut() {
            for num in row.iter_mut() {
                *num += 1;
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                rec(&mut matrix, i, j);
            }
        }

        step += 1;

        let mut flash = true;
        'f: for row in &matrix {
            for num in row {
                if *num != 0 {
                    flash = false;
                    break 'f;
                }
            }
        }

        if flash {
            println!("{}", step);
            break;
        }
    }
}

fn rec(matrix: &mut [Vec<u8>], i: usize, j: usize) {
    if matrix[i][j] < 10 {
        return;
    }

    matrix[i][j] = 0;

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
            rec(matrix, i2 as usize, j2 as usize);
        }
    }
}
