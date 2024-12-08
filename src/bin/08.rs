use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

        let mut x_max = 0;
        let mut y_max = 0;
        for (y_index, line) in reader.lines().enumerate() {
            let line = line?;
            y_max = y_index;
            for (x_index, char) in line.chars().enumerate() {
                x_max = x_index;
                if char != '.' {
                    antennas
                        .entry(char)
                        .or_default()
                        .push((x_index as i64, y_index as i64));
                }
            }
        }

        let mut antinodes = HashSet::new();

        for entry in antennas {
            let frequency = entry.1;
            for (a0, a1) in frequency.iter().tuple_combinations() {
                let dx = a0.0 - a1.0;
                let dy = a0.1 - a1.1;
                antinodes.insert((a0.0 + dx, a0.1 + dy));
                antinodes.insert((a1.0 - dx, a1.1 - dy));
            }
        }

        for antinode_position in antinodes {
            if 0 <= antinode_position.0
                && antinode_position.0 <= x_max as i64
                && 0 <= antinode_position.1
                && antinode_position.1 <= y_max as i64
            {
                answer += 1;
            }
        }
        Ok(answer)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

        let mut x_max = 0;
        let mut y_max = 0;
        for (y_index, line) in reader.lines().enumerate() {
            let line = line?;
            y_max = y_index;
            for (x_index, char) in line.chars().enumerate() {
                x_max = x_index;
                if char != '.' {
                    antennas
                        .entry(char)
                        .or_default()
                        .push((x_index as i64, y_index as i64));
                }
            }
        }

        let mut antinodes = HashSet::new();

        for entry in antennas {
            let frequency = entry.1;
            for (a0, a1) in frequency.iter().tuple_combinations() {
                let dx = a0.0 - a1.0;
                let dy = a0.1 - a1.1;

                let mut steps = 0;
                loop {
                    let tx = a0.0 + steps * dx;
                    let ty = a0.1 + steps * dy;
                    if 0 <= tx && tx <= x_max as i64 && 0 <= ty && ty <= y_max as i64 {
                        antinodes.insert((tx, ty));
                    } else {
                        break;
                    }
                    steps += 1;
                }
                steps = 1;
                loop {
                    let tx = a0.0 - steps * dx;
                    let ty = a0.1 - steps * dy;
                    if 0 <= tx && tx <= x_max as i64 && 0 <= ty && ty <= y_max as i64 {
                        antinodes.insert((tx, ty));
                    } else {
                        break;
                    }
                    steps += 1;
                }
            }
        }
        let answer = antinodes.len();
        Ok(answer)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
