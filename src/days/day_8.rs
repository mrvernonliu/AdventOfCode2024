use std::collections::{HashMap, HashSet};
use std::fs;
use log::{debug, info};
use crate::utils::position::Position;

pub fn part_1() {
    // The pattern seems to be sum(x = n*(n+1))
    // But unfortunately we can't just apply this math
    // because it can be out of bounds and those don't count
    let (map_of_antennas, grid_boundary) = load_input();
    debug!("Map of antennas: {:?}", map_of_antennas);

    let mut antinode_position_set: HashSet<(i32, i32)> = HashSet::new();
    map_of_antennas.iter().for_each(|(c, locations)| {
        let antinode_positions = calculate_antinodes(locations, &grid_boundary);

        debug!("Locations for {}: {:?}", c, locations);
        debug!("Antinode positions for {}: {:?}", c, antinode_positions);
        antinode_position_set.extend(antinode_positions);
    });
    debug!("Antinode positions: {:?}", antinode_position_set);
    info!("Part 1: {}", antinode_position_set.len());
}

pub fn part_2() {
    let (map_of_antennas, grid_boundary) = load_input();
    debug!("Map of antennas: {:?}", map_of_antennas);

    let mut antinode_position_set: HashSet<(i32, i32)> = HashSet::new();
    map_of_antennas.iter().for_each(|(c, locations)| {
        let antinode_positions = calculate_antinodes_with_harmonics(locations, &grid_boundary);

        debug!("Locations for {}: {:?}", c, locations);
        debug!("Antinode positions for {}: {:?}", c, antinode_positions);
        antinode_position_set.extend(antinode_positions);
    });
    debug!("{}", to_grid_string(&map_of_antennas, &antinode_position_set, &grid_boundary));
    debug!("Antinode positions: {:?}", antinode_position_set);
    info!("Part 2: {}", antinode_position_set.len());
}

fn calculate_antinodes(locations: &HashSet<Position<i32>>, boundary: &Position<i32>) -> HashSet<(i32, i32)> {
    let mut antinode_position_set: HashSet<(i32, i32)> = HashSet::new();
    locations.iter().enumerate().for_each(|(i, entry)| {
        locations.iter().enumerate().for_each(|(j, entry_2)| {
            if i != j {
                let delta = entry.subtract(entry_2);
                let antinode_position = entry.add(&delta);
                if !is_out_of_bounds(&antinode_position, boundary) {
                    antinode_position_set.insert((antinode_position.x, antinode_position.y));
                }
            }
        })
    });
    antinode_position_set
}

fn calculate_antinodes_with_harmonics(locations: &HashSet<Position<i32>>, boundary: &Position<i32>) -> HashSet<(i32, i32)> {
    let mut antinode_position_set: HashSet<(i32, i32)> = HashSet::new();
    locations.iter().enumerate().for_each(|(i, entry)| {
        locations.iter().enumerate().for_each(|(j, entry_2)| {
            if i != j {
                let delta = entry.subtract(entry_2);
                let mut antinode_position = entry.add(&delta);
                // Harmonic will appear at the original antenna location to create the line.
                // Another option is to remove the i == j check, but with this implementation
                // that will cause an infinite loop with the boundary check.
                // Hardcoding this logic is good enough.
                antinode_position_set.insert((entry.x, entry.y));
                while !is_out_of_bounds(&antinode_position, boundary) {
                    antinode_position_set.insert((antinode_position.x, antinode_position.y));
                    antinode_position = antinode_position.add(&delta);
                }
            }
        })
    });
    antinode_position_set
}

fn is_out_of_bounds(position: &Position<i32>, boundary: &Position<i32>) -> bool {
    position.x >= boundary.x || position.y >= boundary.y || position.x < 0 || position.y < 0
}

// Debug function
fn to_grid_string(antenna_map: &HashMap<char, HashSet<Position<i32>>>, antinode_set: &HashSet<(i32, i32)>, boundary: &Position<i32>) -> String {
    let mut output = String::new();
    for y in 0..boundary.y {
        for x in 0..boundary.x {
            let mut found = false;
            for (c, locations) in antenna_map {
                if locations.contains(&Position::new(x, y)) {
                    if antinode_set.contains(&(x, y)) {
                        output.push('X')
                    } else {
                        output.push(*c);
                    }
                    found = true;
                    break;
                }
            }
            if !found && antinode_set.contains(&(x, y)) {
                output.push('#');
            } else if !found {
                output.push('.');
            }
        }
        output.push('\n');
    }
    debug!("Count grid shows: {}", output.matches('#').count());
    output
}

pub fn load_input() -> (HashMap<char, HashSet<Position<i32>>>, Position<i32>) {
    let input = fs::read_to_string("./resources/day8.txt")
        .expect("Failed to read input");
    let grid_boundary: Position<i32> = Position::new(input.lines().next().unwrap().len() as i32, input.lines().count() as i32);

    let mut map_of_antennas: HashMap<char, HashSet<Position<i32>>> = HashMap::new();
    input.lines().enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate().for_each(|(x, c)| {
                if c.is_alphanumeric() {
                    map_of_antennas.entry(c)
                        .or_insert_with(HashSet::new)
                        .insert(Position::new(x as i32, y as i32));
                }
            })
        });
    (map_of_antennas, grid_boundary)
}