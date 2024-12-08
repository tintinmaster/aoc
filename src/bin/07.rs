use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::env::var;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;

            let (tresult, tvars) = line.split(": ").collect_tuple().unwrap();
            let result = tresult.parse::<u64>()?;
            let vars: Vec<u64> = tvars
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec();
            if evaluate(result, &vars, 1, vars[0]) {
                answer += result;
            }
        }
        Ok(answer as usize)
    }
    fn evaluate(result: u64, vars: &Vec<u64>, index: usize, t_result: u64) -> bool {
        if index == vars.len() {
            return result == t_result;
        }

        if t_result > result {
            return false;
        }

        let add_result = t_result + vars[index];
        let mul_result = t_result * vars[index];
        evaluate(result, vars, index + 1, add_result)
            || evaluate(result, vars, index + 1, mul_result)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregio

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;

            let (tresult, tvars) = line.split(": ").collect_tuple().unwrap();
            let result = tresult.parse::<u64>()?;
            let vars: Vec<u64> = tvars
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec();
            if evaluate2(result, &vars, 1, vars[0]) {
                answer += result;
            }
        }
        Ok(answer as usize)
    }
    fn evaluate2(result: u64, vars: &Vec<u64>, index: usize, t_result: u64) -> bool {
        if index == vars.len() {
            return result == t_result;
        }

        if t_result > result {
            return false;
        }

        let add_result = t_result + vars[index];
        let mul_result = t_result * vars[index];
        let concat_result = {
            let mut temp_right = vars[index];
            let mut temp_val = t_result;
            while temp_right > 0 {
                temp_val *= 10;
                temp_right /= 10;
            }
            temp_val + vars[index]
        };
        evaluate2(result, vars, index + 1, add_result)
            || evaluate2(result, vars, index + 1, mul_result)
            || evaluate2(result, vars, index + 1, concat_result)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
