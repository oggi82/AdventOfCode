use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn check_increasing(list: Vec<i32>) -> bool {
    for (i, &x) in list.iter().enumerate() {
        // skip the first one so we can guarantee that there's a "previous"
        if i == 0 {
            continue;
        }

        let current = x;
        let previous = list[i - 1];

        if current <= previous {
            return false;
        }

        if current - previous > 3 {
            return false;
        }
    }

    true
}

fn check_decreasing(list: Vec<i32>) -> bool {
    for (i, &x) in list.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let current = x;
        let previous = list[i - 1];

        if current >= previous {
            return false;
        }

        if previous - current > 3 {
            return false;
        }
    }

    true
}

fn generate_vecs(list: Vec<i32>) -> Vec<Vec<i32>> {
    let len = list.len();
    let mut vex: Vec<Vec<i32>> = Vec::new();

    for s in 0..len {
        let mut local_vec: Vec<i32> = Vec::new();
        for (i, n) in list.iter().enumerate() {
            if s == i {
                continue;
            }

            local_vec.push(*n);
        }

        vex.push(local_vec);
    }

    vex
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut counter = 0;
        let vec = generate_vector(reader);
        for v in vec {
            let inc = check_increasing(v.clone());
            let dec = check_decreasing(v.clone());
            if inc || dec {
                counter += 1;
                continue;
            }
        }
        Ok(counter)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result); // 218
                                     //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut counter = 0;
        let vex = generate_vector(reader);
        for v in vex {
            let inc = check_increasing(v.clone());
            let dec = check_decreasing(v.clone());
            if inc || dec {
                counter += 1;
                continue;
            }
            let vecs = generate_vecs(v.clone());
            for l in vecs {
                let inc = check_increasing(l.clone());
                let dec = check_decreasing(l.clone());

                if inc || dec {
                    // we have found a list that works, no need to check the rest
                    counter += 1;
                    break;
                }
            }
        }
        Ok(counter)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn generate_vector<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    let lines = reader.lines().map(|w| w.unwrap()).collect::<Vec<_>>();
    let vex = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    vex
}
