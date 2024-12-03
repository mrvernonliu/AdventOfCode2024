use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn part_1() {
    let (column1, column2) = load_input();
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();

    for i in 0..column1.len() {
        heap1.push(Reverse(column1[i]));
        heap2.push(Reverse(column2[i]));
    }

    let mut differences = Vec::new();
    while !heap1.is_empty() {
        let val1 = heap1.pop().unwrap();
        let val2 = heap2.pop().unwrap();
        println!("{} - {}", val1.0, val2.0); // Reverse() has the worst notation ever
        differences.push((val1.0 - val2.0).abs());
    }
    let sum = differences.iter().sum::<i32>();
    println!("Part 1 - Sum: {}", sum);
}

pub fn part_2() {
    let (column1, column2) = load_input();
    let mut occurrence_map = HashMap::new();
    column2.iter().for_each(|&val| {
        occurrence_map
            .entry(val)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });
    let mut result: i32 = 0;
    column1.iter().for_each(|&val| {
        let multiplier: i32 = match occurrence_map.get(&val) {
            Some(count) => *count,
            None => 0
        };
        result += val * multiplier;
    });
    println!("Part 2: {}", result)
}

fn load_input() -> (Vec<i32>, Vec<i32>) {
    let path = Path::new("resources/day1.txt");
    if !path.exists() {
        panic!("File not found: {:?}", path.to_str());
    }
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();
        if let (Some(col1), Some(col2)) = (parts.next(), parts.next()) {
            column1.push(col1.parse::<i32>().expect("Failed to parse column 1"));
            column2.push(col2.parse::<i32>().expect("Failed to parse column 2"));
        }
    }
    (column1, column2)
}