use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<u32> {
        let re = Regex::new(r"mul\(\d+,\d+\)")?;
        let mut answer = 0;

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let s = std::str::from_utf8(&buffer);
        for x in re.find_iter(s.unwrap()) {
            let (a,b) = x.as_str()[4..x.len()-1].split_once(',').unwrap();
            let i = a.parse::<u32>()? * b.parse::<u32>()?;
            answer += i;
        }

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    assert_eq!(174103751, result);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(mut reader: R) -> Result<u32> {
        let re = Regex::new(r"mul\(\d+,\d+\)|don't|do")?;
        let mut answer = 0;
        let mut active : bool = true;
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let s = std::str::from_utf8(&buffer);
        for x in re.find_iter(s.unwrap()) {
            match x.as_str() {
                "do" => active = true,
                "don't" => active = false,
                _ => {
                    if active {
                        let (a,b) = x.as_str()[4..x.len()-1].split_once(',').unwrap();
                        let i = a.parse::<u32>()? * b.parse::<u32>()?;
                        answer += i;
                    }
                }
            }
        }
        Ok(answer)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
