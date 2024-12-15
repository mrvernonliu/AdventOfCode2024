use std::collections::{HashMap, VecDeque};
use std::fs;
use log::{debug, info};
use num_bigint::BigUint;

const BLINK_COUNT_PART1: i32 = 25;
const BLINK_COUNT_PART2: i32 = 75;
pub fn part_1() {
    let mut result_list: VecDeque<u64>  = load_input();
    debug!("Part 1: {:?}", result_list);

    for _ in 0..BLINK_COUNT_PART1 {
        let mut intermediate_list: VecDeque<u64> = VecDeque::new();
        while let Some(mut node) = result_list.pop_front() {
            if node == 0 {
                debug!("Node is 0, setting as 1");
                node = 1;
                intermediate_list.push_back(node);
            } else if is_even_digit_count(node) {
                debug!("Node {} is even digit count, splitting", node);
                let (node1, node2) = split_node(node);
                intermediate_list.push_back(node1);
                intermediate_list.push_back(node2);
            } else {
                debug!("Node {} does not match anything, multiplying by 2024", node);
                node *= 2024;
                intermediate_list.push_back(node);
            }
        }
        result_list = intermediate_list;
        debug!("Result List: {:?}", result_list);
    }
    info!("Part 1: {}", result_list.len());
}

pub fn part_2() {
    let mut result_list: VecDeque<u64>  = load_input();
    let mut result_map: HashMap<u64, BigUint> = HashMap::new();
    while let Some(node) = result_list.pop_front() {
        *result_map.entry(node).or_insert(BigUint::ZERO) += BigUint::from(1u16);
    }

    debug!("Part 1: {:?}", result_list);

    for _ in 0..BLINK_COUNT_PART2 {
        let mut intermediate_map: HashMap<u64, BigUint> = HashMap::new();
        for (node, count) in result_map.iter() {
            if *node == 0 {
                debug!("Node is 0, setting as 1");
                *intermediate_map.entry(1).or_insert(BigUint::ZERO) += count;
            } else if is_even_digit_count(*node) {
                debug!("Node {} is even digit count, splitting", node);
                let (node1, node2) = split_node(*node);
                *intermediate_map.entry(node1).or_insert(BigUint::ZERO) += count;
                *intermediate_map.entry(node2).or_insert(BigUint::ZERO) += count;
            } else {
                debug!("Node {} does not match anything, multiplying by 2024", node);
                *intermediate_map.entry(*node * 2024).or_insert(BigUint::ZERO) += count;
            }
        }
        result_map = intermediate_map;
        debug!("Result map: {:?}", result_map);
    }
    let sum: BigUint = result_map.iter().map(|(_, count)| {
        count
    }).sum();
    info!("Part 2: {}", sum);
}

fn is_even_digit_count(node: u64) -> bool {
    format!("{}", node).len()%2 == 0
}

fn split_node(node: u64) -> (u64, u64) {
    let node_str = format!("{}", node);
    let (first, second) = node_str.split_at(node_str.len()/2);
    (first.parse().unwrap(), second.parse().unwrap())
}

fn load_input() -> VecDeque<u64> {
    let input_string = fs::read_to_string("./resources/day11.txt").expect("Failed to read file");
    input_string.split_whitespace().map(|s| {
        s.parse().unwrap()
    }).collect()
}