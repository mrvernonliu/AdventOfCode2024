use std::collections::HashSet;
use std::fs;
use std::ops::Add;
use log::{debug, info};

#[derive(Debug)]
struct Field {
    area: i32,
    perimeter: i32
}

impl Field {
    pub fn new(area: i32, perimeter: i32 ) -> Field {
        Field { area, perimeter }
    }
}

impl Add for Field {
    type Output = Field;

    fn add(self, rhs: Self) -> Self::Output {
        Field {
            area: self.area + rhs.area,
            perimeter: self.perimeter + rhs.perimeter,
        }
    }
}
pub fn part_1() {
    let mut grid = load_input();
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
                field_list.push(area_and_perimeter_dfs(&mut grid, x as i32, y as i32, field_type, &mut total_calculated_positions, &mut current_field_positions));
            }
        }
    }

    let price: i32 = field_list.iter().map(|field| {
        field.area * field.perimeter
    }).sum();
    info!("Part 1: {}", price);
}

fn area_and_perimeter_dfs(grid: &mut Vec<Vec<char>>, x: i32, y: i32, field_type: char, total_calculated_positions: &mut HashSet<(i32, i32)>, current_field_positions: &mut HashSet<(i32, i32)>) -> Field {
    if is_out_of_bounds(grid, x, y) {
        return Field::new(0, 1)
    }
    if current_field_positions.contains(&(x, y)) {
        return Field::new(0,0)
    }
    if grid[y as usize][x as usize] != field_type {
        return Field::new(0, 1)
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

fn is_out_of_bounds(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32
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