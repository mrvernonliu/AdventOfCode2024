use std::collections::HashSet;
use std::fs;
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
                sum += dfs(&grid, x as i32, y as i32, -1, &mut visited_trailheads);
            }
        }
    }
    info!("Part 1: {}", sum);
}

fn dfs(grid: &Vec<Vec<char>>, x: i32, y: i32, prev_val: i32, visited_trailheads: &mut HashSet<(i32, i32)>) -> i32 {
    if is_out_of_bounds(grid, x, y) {
        debug!("Out of bounds at {}, {}", x, y);
        return 0;
    }
    if grid[y as usize][x as usize] == '.' {
        return 0;
    }

    let grid_value = grid[y as usize][x as usize].to_digit(10).unwrap() as i32;
    if grid_value - prev_val != 1 {
        return 0;
    }
    if prev_val == 8 && grid_value == 9 {
       if visited_trailheads.contains(&(x, y)) {
           return 0;
       } else {
           visited_trailheads.insert((x, y));
           return 1;
       }
    }
    let mut count = 0;
    count += dfs(grid, x + 1, y, grid_value, visited_trailheads);
    count += dfs(grid, x - 1, y, grid_value, visited_trailheads);
    count += dfs(grid, x, y + 1, grid_value, visited_trailheads);
    count += dfs(grid, x, y - 1, grid_value, visited_trailheads);
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
                sum += dfs_part2(&grid, x as i32, y as i32, -1);
            }
        }
    }
    info!("Part 2: {}", sum);
}

fn dfs_part2(grid: &Vec<Vec<char>>, x: i32, y: i32, prev_val: i32) -> i32 {
    if is_out_of_bounds(grid, x, y) {
        debug!("Out of bounds at {}, {}", x, y);
        return 0;
    }
    if grid[y as usize][x as usize] == '.' {
        return 0;
    }

    let grid_value = grid[y as usize][x as usize].to_digit(10).unwrap() as i32;
    if grid_value - prev_val != 1 {
        return 0;
    }
    if prev_val == 8 && grid_value == 9 {
        return 1;
    }
    let mut count = 0;
    count += dfs_part2(grid, x + 1, y, grid_value);
    count += dfs_part2(grid, x - 1, y, grid_value);
    count += dfs_part2(grid, x, y + 1, grid_value);
    count += dfs_part2(grid, x, y - 1, grid_value);
    count
}


fn is_out_of_bounds(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32
}

fn load_input() -> Vec<Vec<char>> {
    let input_string = fs::read_to_string("./resources/day10.txt").expect("failed to load input");
    let mut grid: Vec<Vec<char>> = Vec::new();
    input_string.lines().for_each(|line| {
        let mut line_vec: Vec<char> = Vec::new();
        line.chars().for_each(|c| {
            line_vec.push(c);
        });
        grid.push(line_vec)
    });
    grid
}