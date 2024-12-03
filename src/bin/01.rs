use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let (mut list1, mut list2): (Vec<usize>, Vec<usize>) = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                Some((
                    parts.next()?.parse::<usize>().unwrap(),
                    parts.next()?.parse::<usize>().unwrap(),
                ))
            })
            .unzip();
        list1.sort();
        list2.sort();

        let mut answer = 0;
        for i in 0..list1.len() {
            answer += list1[i].abs_diff(list2[i]);
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut counter1: HashMap<usize, usize> = HashMap::new();
        let mut counter2: HashMap<usize, usize> = HashMap::new();
        for line in reader.lines() {
            let (val1, val2) = line?
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            *counter1.entry(val1).or_insert(0) += 1;
            *counter2.entry(val2).or_insert(0) += 1;
        }

        let mut answer = 0;
        for (elem, count) in &counter1 {
            answer += *count * (*elem * counter2.get(elem).unwrap_or(&0));
        }

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
