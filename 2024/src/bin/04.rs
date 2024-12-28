use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

type Grid = Vec<Vec<char>>;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

struct Puzzle {
    grid : Grid
}

impl Puzzle {
    fn word_count(&self, word: &str) -> u32 {
        let start = word.chars().next().unwrap();

        self.positions()
            .filter(|&(x, y)| self.char_at(x, y) == Some(start))
            .flat_map(|(x, y)| self.words_from(x, y, word.len()))
            .filter(|found_word| found_word == word)
            .count() as u32
    }

    // Just hardcode we're looking for the MAS cross
    fn cross_count(&self) -> u32 {
        self.positions()
            .filter(|&(x, y)| self.char_at(x, y) == Some('A'))
            .filter(|&(x, y)| {
                let lt = self.char_at(x - 1, y - 1);
                let rt = self.char_at(x + 1, y - 1);
                let lb = self.char_at(x - 1, y + 1);
                let rb = self.char_at(x + 1, y + 1);

                matches!(
                    (lt, rb, lb, rt),
                    (Some('M'), Some('S'), Some('M'), Some('S')) |
                    (Some('S'), Some('M'), Some('S'), Some('M')) |
                    (Some('M'), Some('S'), Some('S'), Some('M')) |
                    (Some('S'), Some('M'), Some('M'), Some('S'))
                )
            })
            .count() as u32
    }

    fn words_from(&self, x: isize, y: isize, len: usize) -> impl Iterator<Item = String> + '_ {
        DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
            (0..len as isize)
                .map(|n| self.char_at(x + dx * n, y + dy * n))
                .collect()
        })
    }

    fn char_at(&self, x: isize, y: isize) -> Option<char> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn positions(&self) -> impl Iterator<Item = (isize, isize)> {
        let rows = self.grid.len() as isize;
        let cols = self.grid.first().map_or(0, |row| row.len()) as isize;
        (0..rows).flat_map(move |x| (0..cols).map(move |y| (x, y)))
    }
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let grid: Grid = input.lines().map(|l| l.chars().collect() ).collect();

        Self { grid }
    }
}

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; 

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<u32> {
        let mut s = String::new();
        let _ = reader.read_to_string(&mut s);
        let answer = Puzzle::from(s.as_str()).word_count("XMAS");
        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(mut reader: R) -> Result<u32> {
        let mut s = String::new();
        let _ = reader.read_to_string(&mut s);
        let answer = Puzzle::from(s.as_str()).cross_count();
        Ok(answer)
    }
    
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}