use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut numbers = Vec::<i32>::new();
    let mut boards = Vec::<[[i32; 5]; 10]>::new();
    let mut board_id = 0;
    let mut row_id = 0;
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            for number in line.split(',') {
                numbers.push(number.parse().unwrap())
            }
            continue;
        }

        if line.is_empty() {
            if i != 1 {
                for i in 5..10 {
                    let mut row = [0; 5];
                    for j in 0..5 {
                        row[j] = boards[board_id][j][i - 5];
                    }
                    boards[board_id][i] = row;
                }
                board_id += 1;
            }
            row_id = 0;
            boards.push([[0; 5]; 10]);
            continue;
        }

        let mut row = [0; 5];
        for (i, num) in line.split_whitespace().enumerate() {
            row[i] = num.parse().unwrap();
        }
        boards[board_id][row_id] = row;
        row_id += 1;
    }

    for i in 5..10 {
        let mut row = [0; 5];
        for j in 0..5 {
            row[j] = boards[board_id][j][i - 5];
        }
        boards[board_id][i] = row;
    }

    let mut result = 0;
    for expect in numbers {
        for board in boards.iter_mut() {
            let mut win = false;
            for row in board.iter_mut() {
                let mut sum = 0;
                for num in row.iter_mut() {
                    if *num == expect {
                        *num = -1;
                    }
                    sum += *num;
                }

                if sum == -5 {
                    win = true;
                }
            }

            if win {
                let mut sum = 0;
                for row in board.iter().take(5) {
                    for num in row {
                        if *num > 0 {
                            sum += num;
                        }
                    }
                }

                *board = [[-2; 5]; 10];
                result = sum * expect;
            }
        }
    }

    println!("{}", result);
}
