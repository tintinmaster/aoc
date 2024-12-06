use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut ordering: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut read_ordering = true;
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                read_ordering = false;
                continue;
            }
            if read_ordering {
                let (val_a, val_b) = line
                    .split("|")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                ordering.entry(val_a).or_default().push(val_b);
            } else {
                let mut valid = true;
                let print_queue: Vec<usize> = line
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                for i in (1..print_queue.len()).rev() {
                    if !valid {
                        break;
                    }
                    let curr_ord = ordering.get(&print_queue[i]);
                    if curr_ord.is_none() {
                        continue;
                    }
                    for j in 0..i {
                        if curr_ord.unwrap().contains(&print_queue[j]) {
                            valid = false;
                        }
                    }
                }
                if valid {
                    let mid = (print_queue.len() - 1) / 2;
                    answer += print_queue[mid];
                }
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut ordering: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut read_ordering = true;
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                read_ordering = false;
                continue;
            }
            if read_ordering {
                let (val_a, val_b) = line
                    .split("|")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                ordering.entry(val_a).or_default().push(val_b);
            } else {
                let print_queue: Vec<usize> = line
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                answer += check_ordering(&ordering, &print_queue);
            }
        }
        Ok(answer)
    }
    fn check_ordering(ordering: &HashMap<usize, Vec<usize>>, print_queue: &Vec<usize>) -> usize {
        let mut new_print: Vec<usize> = print_queue.clone();
        let mut valid = true;
        for i in (1..print_queue.len()).rev() {
            let curr_ord = ordering.get(&print_queue[i]);
            if curr_ord.is_none() {
                continue;
            }
            for j in 0..i {
                if curr_ord.unwrap().contains(&print_queue[j]) {
                    valid = false;
                    for k in (j..i).rev() {
                        new_print.swap(k, k + 1);
                    }
                }
            }
        }
        if valid {
            0
        } else {
            /*let tmp = check_ordering(ordering, &new_print);
            if tmp == 0 {
             */
            let mid = (print_queue.len() - 1) / 2;
            dbg!(&new_print);
            dbg!(new_print[mid])
            /*} else {
                tmp
            }

             */
        }
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
