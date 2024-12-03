use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Ordering;
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

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            let mut increasing = false;
            let mut decreasing = false;
            let mut not_safe = false;

            for i in 0..numbers.len() - 1 {
                match numbers[i].cmp(&numbers[i + 1]) {
                    Ordering::Greater => decreasing = true,
                    Ordering::Less => increasing = true,
                    Ordering::Equal => not_safe = true,
                }

                if numbers[i].abs_diff(numbers[i + 1]) < 1
                    || numbers[i].abs_diff(numbers[i + 1]) > 3
                {
                    not_safe = true;
                }

                if (increasing && decreasing) || not_safe {
                    break;
                }
            }

            if !((increasing && decreasing) || not_safe) {
                answer += 1;
            }
        }
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            let mut safe = false;

            for i in 0..numbers.len() {
                let numbers_i = [&numbers[..i], &numbers[i + 1..]].concat();

                let mut increasing = false;
                let mut decreasing = false;
                let mut not_safe = false;

                for i in 0..numbers_i.len() - 1 {
                    match numbers_i[i].cmp(&numbers_i[i + 1]) {
                        Ordering::Greater => decreasing = true,
                        Ordering::Less => increasing = true,
                        Ordering::Equal => not_safe = true,
                    }

                    if numbers_i[i].abs_diff(numbers_i[i + 1]) < 1
                        || numbers_i[i].abs_diff(numbers_i[i + 1]) > 3
                    {
                        not_safe = true;
                    }

                    if (increasing && decreasing) || not_safe {
                        break;
                    }
                }

                if !((increasing && decreasing) || not_safe) {
                    safe = true;
                    break;
                }
            }

            if safe {
                answer += 1;
            }
        }
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
