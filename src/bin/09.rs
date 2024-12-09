use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

#[derive(Debug)]
struct Block {
    empty: bool,
    id: usize,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<String>();
        let mut id = 0;
        let mut free = false;
        let mut filesystem = vec![];
        for c in input.chars() {
            let n = c.to_digit(10).unwrap() as usize;
            if free {
                for _ in 0..n {
                    filesystem.push(Block { empty: true, id: 0 });
                }
            } else {
                for _ in 0..n {
                    filesystem.push(Block { empty: false, id })
                }
                id += 1;
            }
            free = !free;
        }

        let mut front = 0;
        let mut back = filesystem.len() - 1;
        loop {
            if front >= back {
                break;
            }
            if !filesystem[front].empty {
                front += 1;
                continue;
            }
            if filesystem[back].empty {
                back -= 1;
                continue;
            }
            filesystem.swap(front, back);
            front += 1;
            back -= 1;
        }

        let answer = filesystem
            .iter()
            .enumerate()
            .map(|(index, block)| -> usize {
                if block.empty {
                    0
                } else {
                    index * block.id
                }
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[derive(Debug)]
    struct FileBlock {
        id: usize,
        start: usize,
        len: usize,
    }

    #[derive(Debug)]
    struct FreeSpace {
        start: usize,
        len: usize,
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<String>();
        let mut id = 0;
        let mut pos = 0;
        let mut free = false;
        let mut files = vec![];
        let mut free_space = vec![];
        for c in input.chars() {
            let len = c.to_digit(10).unwrap() as usize;
            if free {
                free_space.push(FreeSpace {
                    start: pos,
                    len,
                });
            } else {
                files.push(FileBlock {
                    id,
                    start: pos,
                    len,
                });
                id += 1;
            }
            free = !free;
            pos += len;
        }

        for file in files.iter_mut().rev() {
            for free in free_space.iter_mut() {
                if free.len < file.len {
                    continue;
                } else if file.start < free.start {
                    break;
                }
                free.len -= file.len;
                file.start = free.start;
                free.start += file.len;
                break;
            }
        }

        let answer = files
            .iter()
            .map(|file| -> usize {
                let mut sum = 0;
                for i in file.start..file.start + file.len {
                    sum += file.id * i;
                }
                sum
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
