use std::fs;
use log::{debug, info};
use crate::utils::direction::Direction;
use crate::utils::location::Position;

const CHARACTER_SHAPES: [char; 4] = ['v', '^', '<', '>'];
struct Character {
    direction: Direction,
    position: Position,
}
impl Character {
    fn new (starting_position: Position, shape: char) -> Self {
        let direction = match shape {
            'v' => Direction::SOUTH,
            '^' => Direction::NORTH,
            '<' => Direction::WEST,
            '>' => Direction::EAST,
            _ => panic!("Invalid shape")
        };
        Self {
            direction,
            position: starting_position
        }
    }

    fn get_shape(&self) -> char {
        match self.direction {
            Direction::NORTH => '^',
            Direction::SOUTH => 'v',
            Direction::WEST => '<',
            Direction::EAST => '>',
            _ => panic!("Invalid direction")
        }
    }

    fn rotate_clockwise(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
            _ => panic!("Non-supported direction detected")
        }
    }

    fn get_next_position(&self, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        let (next_x, next_y) = self.direction.apply_movement(self.position.x as i32, self.position.y as i32);
        if next_x < 0 || next_y < 0 {
            return None
        }
        if next_x >= grid[0].len() as i32 || next_y >= grid.len() as i32 {
            return None
        }
        Some((next_x as usize, next_y as usize))
    }
}

const EMPTY_SHAPE: char = '.';
const VISITED_SHAPE: char = 'o';
const BLOCKING_SHAPE: char = '#';

pub fn part_1() {
    let (mut grid, mut character) = load_input();

    let mut count = 0;
    loop {
        debug!("{}", grid_to_string(&grid, &character));
        info!("Player Location: {:?}", character.position);

        if grid[character.position.y][character.position.x] == EMPTY_SHAPE {
            count += 1;
        }
        grid[character.position.y][character.position.x] = VISITED_SHAPE;

        let (x, y) = match character.get_next_position(&grid) {
            None => break,
            Some((x,y)) => (x,y)
        };
        match grid[y][x] {
            BLOCKING_SHAPE => {
                character.rotate_clockwise();
                continue
            }
            EMPTY_SHAPE | VISITED_SHAPE => {
                character.position.x = x;
                character.position.y = y;
            }
            _ => panic!("Unknown shape detected")
        }
    }
    info!("Part 1: {}", count);
}

pub fn part_2() {
    // Attempt to place a blocker at each position in the valid path.
    // Use a 2 pointer solution with the tail moving slower. If the head ever == tail
    // then we have a loop.

    // This is potentially very slow, but not sure if there is a better way
}

fn grid_to_string(grid: &Vec<Vec<char>>, character: &Character) -> String {
    let mut result = String::new();
    for (y, row) in grid.iter().enumerate() {
        let mut line = String::new();
        for (x, &element) in row.iter().enumerate() {
            if y == character.position.y && x == character.position.x {
                line.push(character.get_shape());
            } else {
                line.push(element);
            }
        }
        result.push('\n');
        result.push_str(&line);
        result.push('\n');
    }
    result
}

fn load_input() -> (Vec<Vec<char>>, Character) {
    let mut starting_location: Option<Position> = None;
    let mut starting_shape: Option<char> = None;

    let grid: Vec<Vec<char>> = match fs::read_to_string("./resources/day6.txt") {
        Err(_) => panic!("Could not load file"),
        Ok(input) =>{
            input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    return line
                        .chars()
                        .enumerate()
                        .map(|(x, c)| {
                            if CHARACTER_SHAPES.contains(&c) {
                                starting_location = Some(Position::new(x, y));
                                starting_shape = Some(c);
                                return EMPTY_SHAPE
                            }
                            c
                        })
                        .collect()
                })
                .collect()
        }
    };
    match (starting_location, starting_shape) {
        (Some(location), Some(shape)) => (grid, Character::new(location, shape)),
        _ => panic!("Could not find starting location or shape"),
    }
}