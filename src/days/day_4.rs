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
    y: i32,
    name: COMPASS
}

impl Direction {
    fn apply_movement(&self, x: i32, y: i32) -> (i32, i32) {
        ((x + self.x), (y + self.y))
    }
    const NORTH: Self = Self { x: 0, y: 1, name: COMPASS::N };
    const SOUTH: Self = Self { x: 0, y: -1, name: COMPASS::S};
    const WEST: Self = Self { x: -1, y: 0, name: COMPASS::W };
    const EAST: Self = Self { x: 1, y: 0, name: COMPASS::E };
    const NORTH_EAST: Self = Self { x: 1, y: 1, name: COMPASS::NE };
    const NORTH_WEST: Self = Self { x: -1, y: 1, name: COMPASS::NW };
    const SOUTH_EAST: Self = Self { x: 1, y: -1, name: COMPASS::SE };
    const SOUTH_WEST: Self = Self { x: -1, y: -1, name: COMPASS::SW };

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

    fn get_cross_directional_map() -> HashMap<COMPASS, Vec<Direction>> {
        let mut map = HashMap::new();
        map.insert(COMPASS::N, vec![Direction::NORTH_EAST, Direction::NORTH_WEST]);
        map.insert(COMPASS::S, vec![Direction::SOUTH_EAST, Direction::SOUTH_WEST]);
        map.insert(COMPASS::W, vec![Direction::NORTH_WEST, Direction::SOUTH_WEST]);
        map.insert(COMPASS::E, vec![Direction::NORTH_EAST, Direction::SOUTH_EAST]);
        map
    }
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

#[derive(Eq, PartialEq)]
enum RotationalSearchState {
    MMatch,
    SMatch,
    NoMatch
}

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
    let cross_directional_map = Direction::get_cross_directional_map();
    let mut christmas_count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'A' {
                christmas_count += search_for_real_xmas(&grid, x as i32, y as i32, &cross_directional_map);
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

fn search_for_real_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32, cross_directional_map: &HashMap<COMPASS, Vec<Direction>>) -> i32{
    let mut count = 0;
    let mut result_map: HashMap<&COMPASS, RotationalSearchState> = HashMap::new();
    for (direction_name, search_locations) in cross_directional_map {
        result_map.insert(direction_name, search_direction(grid, x, y, &search_locations));
    }

    count +=  check_and_count_direction(&result_map, &COMPASS::N, &COMPASS::S);
    count += check_and_count_direction(&result_map, &COMPASS::E, &COMPASS::W);
    count
}

fn search_direction(grid: &Vec<Vec<char>>, x: i32, y: i32, search_locations: &Vec<Direction>) -> RotationalSearchState {
    let (new_x1, new_y1) = search_locations.get(0).unwrap().apply_movement(x, y);
    let (new_x2, new_y2) = search_locations.get(1).unwrap().apply_movement(x, y);

    if new_y1 < 0 || new_y1 > grid.len() as i32 -1 || new_x1 < 0 || new_x1 > grid.get(0).unwrap().len() as i32 -1 {
        return RotationalSearchState::NoMatch
    }
    if new_y2 < 0 || new_y2 > grid.len() as i32 -1 || new_x2 < 0 || new_x2 > grid.get(0).unwrap().len() as i32 -1 {
        return RotationalSearchState::NoMatch
    }

    if grid[new_y1 as usize][new_x1 as usize] == grid[new_y2 as usize][new_x2 as usize] {
        match grid[new_y1 as usize][new_x1 as usize] {
            'M' => RotationalSearchState::MMatch,
            'S' => RotationalSearchState::SMatch,
            _ => RotationalSearchState::NoMatch
        }
    } else {
        RotationalSearchState::NoMatch
    }
}

fn check_and_count_direction(rotational_search_map: &HashMap<&COMPASS, RotationalSearchState>, dir1: &COMPASS, dir2: &COMPASS) -> i32 {
    if rotational_search_map.get(dir1).unwrap() != &RotationalSearchState::NoMatch
        && rotational_search_map.get(dir2).unwrap() != &RotationalSearchState::NoMatch {
        if rotational_search_map.get(dir1).unwrap() != rotational_search_map.get(dir2).unwrap() {
            1
        } else {
            0
        }
    } else {
        0
    }
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