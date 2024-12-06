use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        let mut guard = (0, 0);
        let mut guard_facing = (0, -1);

        let mut max_x = 0;
        let mut max_y = 0;
        for (y_index, line) in reader.lines().enumerate() {
            let line = line?;
            max_y = y_index as i32;
            for (x_index, char) in line.chars().enumerate() {
                max_x = x_index as i32;
                match char {
                    '#' => {
                        obstacles.insert((x_index as i32, y_index as i32));
                    }
                    '^' => {
                        guard = (x_index as i32, y_index as i32);
                    }
                    _ => {}
                }
            }
        }

        visited.insert(guard);

        loop {
            let temp_guard = (guard.0 + guard_facing.0, guard.1 + guard_facing.1);
            if obstacles.contains(&temp_guard) {
                guard_facing = face_right(guard_facing);
                continue;
            } else if temp_guard.0 < 0
                || temp_guard.0 > max_x
                || temp_guard.1 < 0
                || temp_guard.1 > max_y
            {
                break;
            }
            guard = temp_guard;
            visited.insert(guard);
        }

        let answer = visited.len();

        Ok(answer)
    }
    fn face_right(old: (i32, i32)) -> (i32, i32) {
        match old {
            (0, -1) => (1, 0),
            (0, 1) => (-1, 0),
            (1, 0) => (0, 1),
            (-1, 0) => (0, -1),
            _ => (0, 0),
        }
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        let mut guard = (0, 0);
        let mut guard_facing = (0, -1);

        let mut max_x = 0;
        let mut max_y = 0;
        for (y_index, line) in reader.lines().enumerate() {
            let line = line?;
            max_y = y_index as i32;
            for (x_index, char) in line.chars().enumerate() {
                max_x = x_index as i32;
                match char {
                    '#' => {
                        obstacles.insert((x_index as i32, y_index as i32));
                    }
                    '^' => {
                        guard = (x_index as i32, y_index as i32);
                    }
                    _ => {}
                }
            }
        }

        let guard_start = guard;
        let guard_facing_start = guard_facing;

        visited.insert(guard);

        loop {
            let temp_guard = (guard.0 + guard_facing.0, guard.1 + guard_facing.1);
            if obstacles.contains(&temp_guard) {
                guard_facing = face_right(guard_facing);
                continue;
            } else if temp_guard.0 < 0
                || temp_guard.0 > max_x
                || temp_guard.1 < 0
                || temp_guard.1 > max_y
            {
                break;
            }
            guard = temp_guard;
            visited.insert(guard);
        }

        let mut answer = 0;
        visited.remove(&guard_start);
        for path_position in visited {
            guard = guard_start;
            guard_facing = guard_facing_start;

            obstacles.insert(path_position);

            let mut loop_visited = HashSet::new();

            loop_visited.insert((guard, guard_facing));

            loop {
                let temp_guard = (guard.0 + guard_facing.0, guard.1 + guard_facing.1);
                if obstacles.contains(&temp_guard) {
                    guard_facing = face_right(guard_facing);
                    continue;
                } else if temp_guard.0 < 0
                    || temp_guard.0 > max_x
                    || temp_guard.1 < 0
                    || temp_guard.1 > max_y
                {
                    break;
                }
                guard = temp_guard;
                if loop_visited.contains(&(guard, guard_facing)) {
                    answer += 1;
                    break;
                }
                loop_visited.insert((guard, guard_facing));
            }

            obstacles.remove(&path_position);
        }
        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
