use std::fs;
use log::info;
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
}

const EMPTY_SHAPE: char = '.';
const VISITED_SHAPE: char = 'o';

pub fn part_1() {
    let (grid, character) = load_input();
    print_grid(&grid, &character);
    info!("Player Location: {:?}", character.position);
}

fn print_grid(grid: &Vec<Vec<char>>, character: &Character) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &element) in row.iter().enumerate() {
            if y == character.position.y && x == character.position.x {
                print!("{}", character.get_shape());
                continue;
            }
            print!("{}", element);
        }
        println!();
    }
}

fn load_input() -> (Vec<Vec<char>>, Character) {
    let mut starting_location: Option<Position> = None;
    let mut starting_shape: Option<char> = None;

    let grid: Vec<Vec<char>> = match fs::read_to_string("./resources/day6_test.txt") {
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