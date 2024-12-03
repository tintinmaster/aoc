use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)")?;

        let input = reader.lines().map_while(Result::ok).collect::<String>();

        let mut answer = 0;
        for result in re.captures_iter(&input) {
            let val1 = result["first"].parse::<i64>()?;
            let val2 = result["second"].parse::<i64>()?;

            answer += val1 * val2;
        }

        Ok(answer as usize)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(
            r"(mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))",
        )?;

        let input = reader.lines().map_while(Result::ok).collect::<String>();

        let mut answer = 0;
        let mut enabled = true;
        for result in re.captures_iter(&input) {
            if result.name("do").is_some() {
                enabled = true;
            } else if result.name("dont").is_some() {
                enabled = false;
            } else {
                let val1 = result["first"].parse::<i64>()?;
                let val2 = result["second"].parse::<i64>()?;

                if enabled {
                    answer += val1 * val2;
                }
            }
        }

        Ok(answer as usize)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
