use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

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
"; // TODO: Add the test input

const TEST2: &str = "\
SBBSBBS
BABABAB
BBMMMBS
SAMXMAS
BBMMMBS
BABABAB
SBBSBBS
";

const TEST3: &str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line_vec = line.chars().collect();
            grid.push(line_vec);
        }

        let max_x = grid[0].len();
        let max_y = grid[1].len();

        let mut answer = 0;
        for y in 0..max_y {
            for x in 0..max_x {
                // Left to Right
                if x <= max_x - 4
                    && grid[y][x].eq(&'X')
                    && grid[y][x + 1].eq(&'M')
                    && grid[y][x + 2].eq(&'A')
                    && grid[y][x + 3].eq(&'S')
                {
                    answer += 1;
                }
                // Right to left
                if x >= 3
                    && grid[y][x].eq(&'X')
                    && grid[y][x - 1].eq(&'M')
                    && grid[y][x - 2].eq(&'A')
                    && grid[y][x - 3].eq(&'S')
                {
                    answer += 1;
                }
                // Top to bottom
                if y <= max_y - 4
                    && grid[y][x].eq(&'X')
                    && grid[y + 1][x].eq(&'M')
                    && grid[y + 2][x].eq(&'A')
                    && grid[y + 3][x].eq(&'S')
                {
                    answer += 1;
                }
                // Bottom to Top
                if y >= 3
                    && grid[y][x].eq(&'X')
                    && grid[y - 1][x].eq(&'M')
                    && grid[y - 2][x].eq(&'A')
                    && grid[y - 3][x].eq(&'S')
                {
                    answer += 1;
                }
                // Top left to bottom right
                if y <= max_y - 4
                    && x <= max_x - 4
                    && grid[y][x].eq(&'X')
                    && grid[y + 1][x + 1].eq(&'M')
                    && grid[y + 2][x + 2].eq(&'A')
                    && grid[y + 3][x + 3].eq(&'S')
                {
                    answer += 1;
                }
                // Bottom right to top left
                if y <= max_y - 4
                    && x <= max_x - 4
                    && grid[y][x].eq(&'S')
                    && grid[y + 1][x + 1].eq(&'A')
                    && grid[y + 2][x + 2].eq(&'M')
                    && grid[y + 3][x + 3].eq(&'X')
                {
                    answer += 1;
                }
                // Bottom left to top right
                if y >= 3
                    && x <= max_x - 4
                    && grid[y][x].eq(&'X')
                    && grid[y - 1][x + 1].eq(&'M')
                    && grid[y - 2][x + 2].eq(&'A')
                    && grid[y - 3][x + 3].eq(&'S')
                {
                    answer += 1;
                }
                // Top right to bottom left
                if y >= 3
                    && x <= max_x - 4
                    && grid[y][x].eq(&'S')
                    && grid[y - 1][x + 1].eq(&'A')
                    && grid[y - 2][x + 2].eq(&'M')
                    && grid[y - 3][x + 3].eq(&'X')
                {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(8, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line_vec = line.chars().collect();
            grid.push(line_vec);
        }

        let max_x = grid[0].len();
        let max_y = grid[1].len();

        let mut answer = 0;
        for y in 1..max_y - 1 {
            for x in 1..max_x - 1 {
                if !grid[y][x].eq(&'A') {
                    continue;
                }

                // Check top left to bottom right
                if ((grid[y - 1][x - 1].eq(&'M') && grid[y + 1][x + 1].eq(&'S'))
                    || (grid[y - 1][x - 1].eq(&'S') && grid[y + 1][x + 1].eq(&'M')))
                    && ((grid[y + 1][x - 1].eq(&'M') && grid[y - 1][x + 1].eq(&'S'))
                        || (grid[y + 1][x - 1].eq(&'S') && grid[y - 1][x + 1].eq(&'M')))
                {
                    answer += 1;
                }
                // Check bottom left to top right
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
