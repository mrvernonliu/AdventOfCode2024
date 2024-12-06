use std::collections::HashMap;
use std::fs;
use log::info;

#[derive(Debug, Eq, PartialEq, Hash)]
enum COMPASS {
    N, E, W, S, NE, NW, SE, SW
}
#[derive(Debug, Eq, PartialEq, Hash)]
struct Direction {
    x: i32,
    y: i32
}

impl Direction {
    fn apply_movement(&self, x: i32, y: i32) -> (i32, i32) {
        ((x + self.x), (y + self.y))
    }
    const NORTH: Self = Self { x: 0, y: 1 };
    const SOUTH: Self = Self { x: 0, y: -1 };
    const WEST: Self = Self { x: -1, y: 0 };
    const EAST: Self = Self { x: 1, y: 0 };

    const NORTH_EAST: Self = Self { x: 1, y: 1 };
    const NORTH_WEST: Self = Self { x: -1, y: 1 };
    const SOUTH_EAST: Self = Self { x: 1, y: -1 };
    const SOUTH_WEST: Self = Self { x: -1, y: -1 };

    const ALL_DIRECTIONS: [Direction; 8] = [
        Direction::NORTH,
        Direction::SOUTH,
        Direction::WEST,
        Direction::EAST,
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ];
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn part_1() {
    let grid = load_input();
    let mut christmas_count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'X' {
                christmas_count += search_for_xmas(&grid, x as i32, y as i32);
            }
        }
    }
    info!("Christmas count: {} ", christmas_count);
}

pub fn part_2() {
    let grid = load_input();
    let mut christmas_count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'A' {
                christmas_count += search_for_real_xmas(&grid, x as i32, y as i32);
            }
        }
    }
    info!("Real Christmas count: {} ", christmas_count);
}

fn search_for_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for direction in Direction::ALL_DIRECTIONS {
        if directional_dfs(grid, 0, &direction, x, y) {
            count += 1;
        }
    }

    count
}

fn directional_dfs(grid: &Vec<Vec<char>>, current_letter_index: usize, direction: &Direction, x: i32, y: i32) -> bool {
    if y < 0 || y > grid.len() as i32 -1 || x < 0 || x > grid.get(0).unwrap().len() as i32 -1 {
        return false
    }
    if current_letter_index == 3 && grid[y as usize][x as usize] == XMAS[3] {
        true
    } else if grid[y as usize][x as usize] == XMAS[current_letter_index] {
        let (new_x, new_y) = direction.apply_movement(x, y);
        directional_dfs(grid, current_letter_index + 1, direction, new_x, new_y)
    } else {
        false
    }
}

fn search_for_real_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32{
    let nw = Direction::NORTH_WEST.apply_movement(x, y);
    let ne = Direction::NORTH_EAST.apply_movement(x, y);
    let sw = Direction::SOUTH_WEST.apply_movement(x, y);
    let se = Direction::SOUTH_EAST.apply_movement(x, y);

    if !check_bounds(&grid, vec![&nw, &ne, &sw, &se]) {
        0
    } else if !check_cross_is_m_and_s(&grid, &nw, &se) {
        0
    } else if !check_cross_is_m_and_s(&grid, &ne, &sw) {
        0
    } else {
        1
    }
}

fn check_cross_is_m_and_s(grid:  &Vec<Vec<char>>, coordinate_1: &(i32, i32), coordinate_2: &(i32, i32)) -> bool {
    let coordinate_1_val = match grid[coordinate_1.1 as usize][coordinate_1.0 as usize] {
        'M' => 'M',
        'S' => 'S',
        _ => return false
    };
    let coordinate_2_val = match grid[coordinate_2.1 as usize][coordinate_2.0 as usize] {
        'M' => 'M',
        'S' => 'S',
        _ => return false
    };
    coordinate_1_val != coordinate_2_val
}

fn check_bounds(grid: &Vec<Vec<char>>, coordinates: Vec<&(i32, i32)>) -> bool {
    coordinates.iter().all(|(x, y)| {
        *x >= 0 && *x < grid[0].len() as i32 && *y >= 0 && *y < grid.len() as i32
    })
}

fn load_input() -> Vec<Vec<char>> {
    let input = match fs::read_to_string("./resources/day4.txt") {
        Ok(input) => input,
        Err(_) => panic!("Failed to read file")
    };

    let len_y: usize = input.lines().count();
    let len_x: usize = input.lines().next().unwrap().chars().count();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; len_x]; len_y];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;
        }
    }
    grid
}