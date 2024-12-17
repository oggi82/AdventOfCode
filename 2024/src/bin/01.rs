use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/",DAY,".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

#[inline]
fn parse_line(line: &str) -> (u32, u32) {
    let (l, r) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
    let l = l.parse().unwrap();
    let r = r.trim_ascii_start().parse().unwrap();
    (l, r)
}

fn main() -> Result<()> {
    start_day(DAY);
    println!("=== part 1 ===");
    
    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let mut left = smallvec::SmallVec::<[u32; 1024]>::new();
        let mut right = smallvec::SmallVec::<[u32; 1024]>::new();
        for line in reader.lines() {
            let (l,r) = parse_line(&line.unwrap());
            left.insert(
                left.binary_search(&l).unwrap_or_else(|e|e), l);
            right.insert(
                right.binary_search(&r).unwrap_or_else(|e|e), r);
        }
        let result:u32 = left.into_iter()
            .zip(right)
            .map(|(l,r)| l.abs_diff(r))
            .sum();
        Ok(result)
    }
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let mut left = smallvec::SmallVec::<[u32; 1024]>::new();
        let mut right = smallvec::SmallVec::<[u32; 1024]>::new();
        for line in reader.lines() {
            let (l,r) = parse_line(&line.unwrap());
            left.insert(
                left.binary_search(&l).unwrap_or_else(|e|e), l);
            right.insert(
                right.binary_search(&r).unwrap_or_else(|e|e), r);
        }

        let mut value : u32 = 0;
        for ele in left {
            let c = right.iter().filter(|&n| *n == ele).count() as u32;
            value += ele * c;
        }
        Ok(value)
    }
    
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2,2);
        assert_eq!(result, 4);
    }
}