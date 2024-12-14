use std::collections::HashSet;
use std::fs;
use std::path::Path;
use log::{debug, info};

pub fn part_1() {
    let grid = load_input();
    debug!("Part 1: {:?}", grid);

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '0' {
                debug!("Found a 0");
                let mut visited_trailheads: HashSet<(i32, i32)> = HashSet::new();
                sum += bfs(&grid, x as i32, y as i32, -1, &mut visited_trailheads);
            }
        }
    }
    info!("Part 1: {}", sum);
}

fn bfs(grid: &Vec<Vec<char>>, x: i32, y: i32, prevVal: i32, visited_trailheads: &mut HashSet<(i32, i32)>) -> i32 {
    if is_out_of_bounds(grid, x, y) {
        debug!("Out of bounds at {}, {}", x, y);
        return 0;
    }
    if grid[y as usize][x as usize] == '.' {
        return 0;
    }

    let gridValue = grid[y as usize][x as usize].to_digit(10).unwrap() as i32;
    if gridValue - prevVal != 1 {
        return 0;
    }
    if prevVal == 8 && gridValue == 9 {
       if visited_trailheads.contains(&(x, y)) {
           return 0;
       } else {
           visited_trailheads.insert((x, y));
           return 1;
       }
    }
    let mut count = 0;
    count += bfs(grid, x + 1, y, gridValue, visited_trailheads);
    count += bfs(grid, x - 1, y, gridValue, visited_trailheads);
    count += bfs(grid, x, y + 1, gridValue, visited_trailheads);
    count += bfs(grid, x, y - 1, gridValue, visited_trailheads);
    count
}

pub fn part_2() {
    let grid = load_input();
    debug!("Part 1: {:?}", grid);

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '0' {
                debug!("Found a 0");
                sum += bfs_part2(&grid, x as i32, y as i32, -1);
            }
        }
    }
    info!("Part 2: {}", sum);
}

fn bfs_part2(grid: &Vec<Vec<char>>, x: i32, y: i32, prevVal: i32) -> i32 {
    if is_out_of_bounds(grid, x, y) {
        debug!("Out of bounds at {}, {}", x, y);
        return 0;
    }
    if grid[y as usize][x as usize] == '.' {
        return 0;
    }

    let gridValue = grid[y as usize][x as usize].to_digit(10).unwrap() as i32;
    if gridValue - prevVal != 1 {
        return 0;
    }
    if prevVal == 8 && gridValue == 9 {
        return 1;
    }
    let mut count = 0;
    count += bfs_part2(grid, x + 1, y, gridValue);
    count += bfs_part2(grid, x - 1, y, gridValue);
    count += bfs_part2(grid, x, y + 1, gridValue);
    count += bfs_part2(grid, x, y - 1, gridValue);
    count
}


fn is_out_of_bounds(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32
}

fn load_input() -> Vec<Vec<char>> {
    let inputString = fs::read_to_string("./resources/day10.txt").expect("failed to load input");
    let mut grid: Vec<Vec<char>> = Vec::new();
    inputString.lines().for_each(|line| {
        let mut lineVec: Vec<char> = Vec::new();
        line.chars().for_each(|c| {
            lineVec.push(c);
        });
        grid.push(lineVec)
    });
    grid
}