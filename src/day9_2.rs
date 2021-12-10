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

    let mut sizes = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            sizes.push(place(&mut matrix, i, j));
        }
    }

    sizes.sort_unstable();
    println!("{}", sizes.iter().rev().take(3).product::<usize>());
}

fn place(matrix: &mut [Vec<u8>], i: usize, j: usize) -> usize {
    if i >= matrix.len() || j >= matrix[0].len() || matrix[i][j] == 9 {
        return 0;
    }

    matrix[i][j] = 9;
    let mut count = 1 + place(matrix, i, j + 1) + place(matrix, i + 1, j);
    if i > 0 {
        count += place(matrix, i - 1, j);
    }
    if j > 0 {
        count += place(matrix, i, j - 1);
    }

    count
}
