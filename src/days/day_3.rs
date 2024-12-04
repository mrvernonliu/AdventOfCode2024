use std::fs;
use log::{debug, info};

const VALID_PROGRAM_REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const DISABLE_KEYWORD: &str = "don't()";
const ENABLE_KEYWORD: &str = "do()";

pub fn part_1() {
    let program = match fs::read_to_string("./resources/day3.txt") {
        Ok(program) => program,
        Err(_) => panic!("Failed to read file")
    };
    debug!("program: {}", program);

    let regex = regex::Regex::new(VALID_PROGRAM_REGEX).unwrap();
    let matches = regex.captures_iter(&program);

    let mut result = 0;
    matches.for_each(|m| {
        let lhs: i32 = m.get(1).unwrap().as_str().parse().expect("Failed to parse lhs");
        let rhs: i32 = m.get(2).unwrap().as_str().parse().expect("Failed to parse rhs");

        result += lhs * rhs;
    });
    info!("Part 1: {}", result);
}

pub fn part_2() {
    let mut program = match fs::read_to_string("./resources/day3.txt") {
        Ok(program) => program,
        Err(_) => panic!("Failed to read file")
    };
    debug!("program: {}", program);

    if !program.contains("don't()") {
        part_1()
    }

    let regex = regex::Regex::new(VALID_PROGRAM_REGEX).unwrap(); // will always exist

    let mut result = 0;
    loop {
        let buckets: Vec<&str> = program.splitn(2,DISABLE_KEYWORD).collect();
        debug!("bucket 1: {}", buckets[0]);
        let bucket_before_disable = regex.captures_iter(buckets[0]);
        bucket_before_disable.for_each(|m| {
            let lhs: i32 = m.get(1).unwrap().as_str().parse().expect("Failed to parse lhs");
            let rhs: i32 = m.get(2).unwrap().as_str().parse().expect("Failed to parse rhs");
            debug!("lhs: {}, rhs: {}", lhs, rhs);
            result += lhs * rhs;
        });

        if buckets.len() > 1 && buckets[1].contains(ENABLE_KEYWORD) {
            let next_bucket: Vec<&str> = buckets[1].splitn(2, ENABLE_KEYWORD).collect();
            if next_bucket.is_empty(){
                break;
            } else {
                program = next_bucket[1].parse().unwrap(); // unwrap because already checked if exists
                debug!("nextBlock: {}", program);
            }
        } else {
            break;
        }
    }
    info!("Part 2: {}", result);
}