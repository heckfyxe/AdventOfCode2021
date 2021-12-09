use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::str;

fn main() {
    let mut file = File::open("input8.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut count = 0u64;
    for line in input.lines() {
        let mut segments = line
            .split('|')
            .rev()
            .last()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        segments.sort_by_key(|a| a.len());

        let mut map = [""; 10];
        let mut i = 0;
        loop {
            if segments[i].len() == 2 {
                map[1] = segments[i];
            }
            if segments[i].len() == 3 {
                map[7] = segments[i];
            }
            if segments[i].len() == 4 {
                map[4] = segments[i];
            }
            if segments[i].len() == 7 {
                map[8] = segments[i];
            }
            if segments[i].len() == 5 {
                let a = *difference(map[7], map[1]).last().unwrap();
                for seg in segments[i..=(i + 2)].iter() {
                    let diff = difference(*seg, map[4]);
                    if diff.iter().filter(|n| **n != a).count() == 2 {
                        map[2] = seg;
                        break;
                    }
                }
                for seg in segments[i..=(i + 2)].iter() {
                    if difference(*seg, map[2]).len() == 2 {
                        map[5] = seg;
                        break;
                    }
                }
                map[3] = segments[i..=(i + 2)]
                    .iter()
                    .filter(|s| **s != map[2])
                    .filter(|s| **s != map[5])
                    .last()
                    .unwrap();

                i += 2;
            }
            if segments[i].len() == 6 {
                for seg in segments[i..].iter() {
                    if difference(*seg, map[5]).len() == 2 {
                        map[0] = *seg;
                        break;
                    }
                }
                for seg in segments[i..].iter() {
                    if *seg != map[0] && difference(*seg, map[3]).len() == 2 {
                        map[6] = *seg;
                        break;
                    }
                }

                map[9] = segments[i..9]
                    .iter()
                    .filter(|s| **s != map[0])
                    .filter(|s| **s != map[6])
                    .last()
                    .unwrap();

                i += 2;
            }

            i += 1;
            if i == 10 {
                break;
            }
        }

        let mut n_map = Vec::<String>::new();
        for item in map {
            n_map.push(sort(item));
        }

        let output: Vec<&str> = line.split('|').last().unwrap().split_whitespace().collect();

        let mut num = 0;
        for n in output {
            let n = sort(n);
            for (i, na) in n_map.iter().enumerate() {
                if *na == n {
                    num *= 10;
                    num += i;
                }
            }
        }

        count += num as u64;
    }
    println!("{}", count);
}

fn difference(first: &str, second: &str) -> Vec<char> {
    let first: HashSet<_> = first.chars().collect();
    let second: HashSet<_> = second.chars().collect();
    first.difference(&second).map(ToOwned::to_owned).collect()
}

fn sort(s: &str) -> String {
    let mut temp = s.chars().collect::<Vec<_>>();
    temp.sort_unstable();
    temp.iter().collect()
}
