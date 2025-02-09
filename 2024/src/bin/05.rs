use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::cmp::Ordering;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = Default::default();
        reader.read_to_string(&mut input).unwrap();
        Ok(evaluate_lines(&input, evaluate_line_part1)
            .try_into()
            .unwrap())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = Default::default();
        reader.read_to_string(&mut input).unwrap();
        Ok(evaluate_lines(&input, evaluate_line_part2)
            .try_into()
            .unwrap())
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

fn evaluate_lines(
    s: &str,
    evaluate_line: fn(&HashMap<i32, HashSet<i32>>, &[i32]) -> Option<i32>,
) -> i32 {
    let (predecessors, lines) = {
        let predecessors = s.lines().take_while(|l| !l.is_empty()).fold(
            HashMap::<_, HashSet<_>>::new(),
            |mut m, line| {
                let (left, right) = line.split_once("|").unwrap();
                let (left, right) = (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap());
                m.entry(right).or_default().insert(left);
                m
            },
        );

        let lines = s
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| {
                line.split(",")
                    .map(i32::from_str)
                    .map(Result::unwrap)
                    .collect_vec()
            })
            .collect_vec();

        (predecessors, lines)
    };
    lines
        .iter()
        .filter_map(|nums| evaluate_line(&predecessors, nums))
        .sum()
}

fn evaluate_line_part1(predecessors: &HashMap<i32, HashSet<i32>>, nums: &[i32]) -> Option<i32> {
    let cmp = |a, b| predecessors.get(b).cloned().unwrap_or_default().contains(a);
    nums.is_sorted_by(|a, b| cmp(a, b))
        .then_some(nums[nums.len() / 2])
}

fn evaluate_line_part2(predecessors: &HashMap<i32, HashSet<i32>>, nums: &[i32]) -> Option<i32> {
    let cmp = |a, b| predecessors.get(b).cloned().unwrap_or_default().contains(a);
    let to_ordering = |b| if b { Ordering::Less } else { Ordering::Greater };
    (!nums.is_sorted_by(|a, b| cmp(a, b))).then(|| {
        *nums
            .iter()
            .sorted_by(|a, b| to_ordering(cmp(a, b)))
            .collect_vec()[nums.len() / 2]
    })
}


