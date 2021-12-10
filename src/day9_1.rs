use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input9.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut matrix = Vec::<Vec<u8>>::new();
    for line in input.lines() {
        let mut vec = Vec::<u8>::new();
        for num in line.chars() {
            let num = num as u8 - b'0';
            vec.push(num);
        }
        matrix.push(vec);
    }
    let n = matrix.len();
    let m = matrix[0].len();

    let mut total = 0u64;
    for (i, row) in matrix.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            let mut c = true;
            c = c && (j == 0 || *num < row[j - 1]); // left
            c = c && (i == 0 || *num < matrix[i - 1][j]); // top
            c = c && (j == m - 1 || *num < row[j + 1]); // right
            c = c && (i == n - 1 || *num < matrix[i + 1][j]); // bottom

            if c {
                total += *num as u64 + 1;
            }
        }
    }

    println!("{}", total);
}
