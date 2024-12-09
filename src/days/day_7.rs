use std::fs;
use log::{debug, info};

pub fn part_1() {
    // Use a recursive tree to check operations (+ or *), terminate immediately if > target
    let input = load_input();
    debug!("Input: {:?}", input);
    let valid_count: i64 = input.iter().filter_map(|(target, terms)| {
        match calculate_recursively(*target, terms[0], &terms, 1) {
            true => Some(target),
            false => None
        }
    }).sum();
    info!("Part 1: {}", valid_count);
}

pub fn part_2() {
    // Use a recursive tree to check operations (+ or * or ||), terminate immediately if > target
    let input = load_input();
    debug!("Input: {:?}", input);
    let valid_count: i64 = input.iter().filter_map(|(target, terms)| {
        match calculate_recursively_with_concat_operation(*target, terms[0], &terms, 1) {
            true => Some(target),
            false => None
        }
    }).sum();
    info!("Part 2: {}", valid_count);
}

fn calculate_recursively(target: i64, current_value: i64, terms: &Vec<i64>, index: usize) -> bool {
    if current_value == target && index == terms.len() {
        return true
    }
    if current_value > target || index >= terms.len() {
        return false
    }

    let addition_case = calculate_recursively(target, current_value + terms[index], terms, index + 1);
    let multiplication_case = calculate_recursively(target, current_value * terms[index], terms, index + 1);
    addition_case || multiplication_case
}

fn calculate_recursively_with_concat_operation(target: i64, current_value: i64, terms: &Vec<i64>, index: usize) -> bool {
    if current_value == target && index == terms.len() {
        return true
    }
    if current_value > target || index >= terms.len() {
        return false
    }

    let addition_case = calculate_recursively_with_concat_operation(target, current_value + terms[index], terms, index + 1);
    let multiplication_case = calculate_recursively_with_concat_operation(target, current_value * terms[index], terms, index + 1);
    let concat_case = calculate_recursively_with_concat_operation(target, concat(current_value, terms[index]), terms, index + 1);
    addition_case || multiplication_case || concat_case
}

// To concat: append the number of digits in y to x
// by multiplying x by 10^(digits in y). Then add Y.
fn concat(x: i64, y: i64) -> i64 {
    let mut num_digits_in_y: u32 = 0;
    let mut copy_y = y;


    while copy_y > 0 {
        copy_y = copy_y / 10;
        num_digits_in_y += 1;
    }

    x * 10_i64.pow(num_digits_in_y) + y
}

fn load_input() -> Vec<(i64, Vec<i64>)> {
    let input = match fs::read_to_string("./resources/day7.txt") {
        Err(_) => panic!("Failed to read input"),
        Ok(input) => input
    };
    input.lines().map(|line| {
        let split: Vec<&str> = line.split(':').collect();
        let lhs: i64 = split[0].parse().expect("Failed to parse lhs as i64");
        let rhs: Vec<i64> = split[1].split_whitespace()
            .map(|x| x.parse().expect("Failed to parse rhs as i64"))
            .collect();
        (lhs, rhs)
    }).collect()

}