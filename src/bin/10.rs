use std::collections::HashSet;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = vec![];
        for (y, line) in reader.lines().enumerate() {
            grid.push(vec![]);
            for char in line?.chars() {
                let height = char.to_digit(10).unwrap();
                grid[y].push(height as u8);
            }
        }
        
        let max_y = grid.len();
        let max_x = grid[0].len();
        
        let mut answer = 0;
        for y in 0..max_y{
            for x in 0..max_x {
                if grid[y][x] == 0 {
                    let res =  search_neighbours(&grid, x as i32, y as i32, max_x as i32, max_y as i32, 1);
                    let res_set: HashSet<&(i32, i32)> = HashSet::from_iter(res.iter());
                    answer += res_set.len();
                }
            }
        }

        Ok(answer)
    }
    fn search_neighbours(grid: &Vec<Vec<u8>>, x: i32, y: i32, max_x: i32, max_y: i32, search_value: u8) -> Vec<(i32, i32)> {
        let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut answer = vec![];
        for (dx, dy) in directions {
            let tx = x as i32 + dx;
            let ty = y as i32 + dy;
            if 0 <= tx && tx < max_x && 0 <= ty && ty < max_y {
               if grid[ty as usize][tx as usize] == search_value {
                   if search_value == 9 {
                       answer.append(vec![(tx, ty)].as_mut());
                   } else {
                       answer.append(search_neighbours(grid, tx, ty, max_x, max_y, search_value + 1).as_mut());
                   }
               } 
            }
        }
        answer
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = vec![];
        for (y, line) in reader.lines().enumerate() {
            grid.push(vec![]);
            for char in line?.chars() {
                let height = char.to_digit(10).unwrap();
                grid[y].push(height as u8);
            }
        }

        let max_y = grid.len();
        let max_x = grid[0].len();

        let mut answer = 0;
        for y in 0..max_y{
            for x in 0..max_x {
                if grid[y][x] == 0 {
                    let res =  search_neighbours(&grid, x as i32, y as i32, max_x as i32, max_y as i32, 1);
                    answer += res.len();
                }
            }
        }

        Ok(answer)
    }
    
    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
