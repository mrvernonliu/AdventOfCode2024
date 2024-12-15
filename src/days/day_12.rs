use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Add;
use log::{debug, info};
use crate::utils::direction::Direction;


const DIRECTIONS: [(i32, i32, Direction); 4] = [
    (1, 0, Direction::EAST),
    (-1, 0, Direction::WEST),
    (0, 1, Direction::SOUTH),
    (0, -1, Direction::NORTH),
];

#[derive(Debug)]
struct Field {
    area: i32,
    perimeter: i32,
    is_edge: bool
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Face {
    direction: Direction,
    parallel_axis: i32
}

impl Field {
    pub fn new(area: i32, perimeter: i32, is_edge: Option<bool>) -> Field {
        Field { area, perimeter, is_edge: is_edge.unwrap_or(false) }
    }
}

impl Add for Field {
    type Output = Field;

    fn add(self, rhs: Self) -> Self::Output {
        Field {
            area: self.area + rhs.area,
            perimeter: self.perimeter + rhs.perimeter,
            is_edge: false
        }
    }
}

pub fn part_1() {
    let grid = load_input();
    debug!("Starting grid: {:?}", grid);

    let mut field_list: Vec<Field> = Vec::new();
    let mut total_calculated_positions: HashSet<(i32, i32)> = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if total_calculated_positions.contains(&(x as i32, y as i32)) {
                continue;
            } else {
                debug!("Checking field at {}, {}", x, y);
                let mut current_field_positions: HashSet<(i32, i32)> = HashSet::new();
                let field_type = grid[y][x];
                field_list.push(area_and_perimeter_dfs(&grid, x as i32, y as i32, field_type, &mut total_calculated_positions, &mut current_field_positions));
            }
        }
    }

    let price: i32 = field_list.iter().map(|field| {
        field.area * field.perimeter
    }).sum();
    info!("Part 1: {}", price);
}

fn area_and_perimeter_dfs(grid: &Vec<Vec<char>>, x: i32, y: i32, field_type: char, total_calculated_positions: &mut HashSet<(i32, i32)>, current_field_positions: &mut HashSet<(i32, i32)>) -> Field {
    if is_out_of_bounds(grid, x, y) {
        return Field::new(0, 1, Some(true))
    }
    if current_field_positions.contains(&(x, y)) {
        return Field::new(0,0, None)
    }
    if grid[y as usize][x as usize] != field_type {
        return Field::new(0, 1, Some(true))
    }
    total_calculated_positions.insert((x, y));
    current_field_positions.insert((x, y));
    let mut field = area_and_perimeter_dfs(grid, x+1, y, field_type, total_calculated_positions, current_field_positions);
    field = field + area_and_perimeter_dfs(grid, x-1, y, field_type, total_calculated_positions, current_field_positions);
    field = field + area_and_perimeter_dfs(grid, x, y-1, field_type, total_calculated_positions, current_field_positions);
    field = field + area_and_perimeter_dfs(grid, x, y+1, field_type, total_calculated_positions, current_field_positions);
    field.area += 1;
    debug!("Position x:{}, y{}, Field: {:?}", x, y, field);
    field
}

pub fn part_2() {
    let grid = load_input();
    debug!("Starting grid: {:?}", grid);

    let mut field_list: Vec<(Field, i32)> = Vec::new();
    let mut total_calculated_positions: HashSet<(i32, i32)> = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if total_calculated_positions.contains(&(x as i32, y as i32)) {
                continue;
            } else {
                debug!("Checking field at {}, {}", x, y);
                let mut current_field_positions: HashSet<(i32, i32)> = HashSet::new();
                let mut face_map: HashMap<Face, Vec<i32>> = HashMap::new();
                let field_type = grid[y][x];
                match area_and_perimeter_face_dfs(&grid, x as i32, y as i32, field_type, &mut total_calculated_positions, &mut current_field_positions, &mut face_map) {
                    None => {}
                    Some(field) => {
                        debug!("Field: {:?}", face_map);
                        let num_sides = calculate_num_sides(face_map);
                        field_list.push((field, num_sides));
                    }
                }
            }
        }
    }

    debug!("Field list: {:?}", field_list);
    let price: i32 = field_list.iter().map(|(field, sides)| {
        field.area * sides
    }).sum();
    info!("Part 1: {}", price);
}

fn area_and_perimeter_face_dfs(grid: &Vec<Vec<char>>, x: i32, y: i32, field_type: char, total_calculated_positions: &mut HashSet<(i32, i32)>, current_field_positions: &mut HashSet<(i32, i32)>, face_map: &mut HashMap<Face, Vec<i32>>) -> Option<Field> {
    if is_out_of_bounds(grid, x, y) {
        return Some(Field::new(0, 1, Some(true)))
    }
    if current_field_positions.contains(&(x, y)) {
        return None
    }
    if grid[y as usize][x as usize] != field_type {
        return Some(Field::new(0, 1, Some(true)))
    }
    total_calculated_positions.insert((x, y));
    current_field_positions.insert((x, y));
    let mut field = Field::new(0,0, None);

    for (dx, dy, direction) in DIRECTIONS {
        match area_and_perimeter_face_dfs(grid, x + dx, y + dy, field_type, total_calculated_positions, current_field_positions, face_map) {
            None => {}
            Some(next_field) => {
                if next_field.is_edge {
                    let new_face = Face { direction, parallel_axis: if dx != 0 { x } else { y } };
                    face_map.entry(new_face).or_insert(Vec::new()).push(if dx != 0 { y } else { x });
                }
                field = field + next_field;
            }
        }
    }

    field.area += 1;
    debug!("Position x:{}, y{}, Field: {:?}", x, y, field);
    Some(field)
}

fn is_out_of_bounds(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32
}

fn calculate_num_sides(face_map: HashMap<Face, Vec<i32>>) -> i32 {
    let mut num_sides = 0;
    for (_, mut positions) in face_map {
        positions.sort();
        for i in 1..positions.len() {
            if positions[i] - positions[i-1] != 1 {
                num_sides += 1;
            }
        }
        num_sides += 1;
    }
    num_sides
}

fn load_input() -> Vec<Vec<char>> {
    let input_string = fs::read_to_string("./resources/day12.txt").expect("failed to load input");
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