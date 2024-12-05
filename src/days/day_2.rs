use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use log::{debug, info};
use crate::days::day_2::SafetyTriggerResult::{Continue, EndOfLine, Terminate};

const ALLOWABLE_DIFFERENCE: i32 = 3;

enum SafetyTriggerResult {
    Terminate,
    Continue,
    EndOfLine,
}

pub fn part_1() {
    let list_of_reports = load_input();
    let valid_report_count = list_of_reports
        .iter()
        .filter(|report| {
            check_if_valid(report)
        })
        .count();
    info!("Part 1: {}", valid_report_count);
}

pub fn part_2() {
    let list_of_reports = load_input();
    let valid_report_count = list_of_reports
        .iter()
        .filter(|report| {
            let forward_result = check_if_valid_with_safety(&report.iter().rev().cloned().collect());
            let reverse_result = check_if_valid_with_safety(report);

            return forward_result || reverse_result;
        })
        .count();
    info!("Part 2 {}", valid_report_count);
}

fn check_if_valid(report: &Vec<i32>) -> bool {
    let mut head = 0;
    let mut tail = 1;

    // Unwrap because problem states that there will always be at least 2 numbers
    let mut head_value = report.get(head).unwrap();
    let mut tail_value = report.get(tail).unwrap();

    let is_ascending = head_value < tail_value;
    loop {
        if is_ascending != (head_value < tail_value) {
            return false;
        }
        let delta = (head_value - tail_value).abs();
        if delta > ALLOWABLE_DIFFERENCE || delta == 0 {
            return false;
        }
        head += 1;
        tail += 1;
        head_value = report.get(head).unwrap();
        tail_value = match report.get(tail) {
            Some(value) => value,
            None => { break; }
        };
    }
    true
}

fn check_if_valid_with_safety(original_report: &Vec<i32>) -> bool {
    debug!("Original Report: {:?}", original_report);
    let mut report = original_report.clone();
    let mut head = 0;
    let mut tail = 1;

    let mut safety_triggered = false;

    let mut head_value = report[head];
    let mut tail_value = report[tail];

    let is_ascending = head_value < tail_value;
    loop {
        debug!("Head: {} - Tail: {}", head_value, tail_value);
        if is_ascending != (head_value < tail_value) {
            match handle_safety_trigger(&mut safety_triggered, &mut report, &tail, &mut tail_value) {
                Terminate => return false,
                EndOfLine => return true,
                Continue => continue,
            }
        }
        let delta = (head_value - tail_value).abs();
        if delta > ALLOWABLE_DIFFERENCE || delta == 0 {
            match handle_safety_trigger(&mut safety_triggered, &mut report, &tail, &mut tail_value) {
                Terminate => return false,
                EndOfLine => return true,
                Continue => continue,
            }
        }
        head += 1;
        tail += 1;
        head_value = *report.get(head).unwrap();
        tail_value = match report.get(tail) {
            Some(value) => *value,
            None => { break; }
        };
    }
    true
}

fn handle_safety_trigger(safety_triggered: &mut bool, report: &mut Vec<i32>, tail: &usize, tail_value: &mut i32) -> SafetyTriggerResult {
    if *safety_triggered {
        debug!("Safety Triggered - termination");
        Terminate
    } else {
        debug!("Safety Triggered");
        *safety_triggered = true;
        report.remove(*tail);
        *tail_value = match report.get(*tail) {
            Some(value) => *value,
            None => return EndOfLine
        };
        Continue //continue
    }
}

fn load_input() -> Vec<Vec<i32>> {
    let path = Path::new("resources/day2.txt");
    if !path.exists() {
        panic!("File not found: {:?}", path.to_str());
    }
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let list_of_reports = reader.lines().map(|line| {
        let line = line.expect("Failed to read line");
        line.split_whitespace()
            .map(|num| num.parse::<i32>().expect("Failed to parse number"))
            .collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    list_of_reports
}