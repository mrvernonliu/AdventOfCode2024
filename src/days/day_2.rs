use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
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
            checkIfValid(report)
        })
        .count();
    println!("Part 1: {}", valid_report_count);
}

pub fn part_2() {
    let list_of_reports = load_input();
    let valid_report_count = list_of_reports
        .iter()
        .filter(|report| {
            let forwardResult = checkIfValidWithSafety(report);
            let reverseResult  = checkIfValidWithSafety(&report.iter().rev().cloned().collect());

            return forwardResult || reverseResult;
        })
        .count();
    println!("Part 2 {}", valid_report_count);
}

fn checkIfValid(report: &Vec<i32>) -> bool {
    let mut head = 0;
    let mut tail = 1;

    // Unwrap because problem states that there will always be at least 2 numbers
    let mut headValue = report.get(head).unwrap();
    let mut tailValue = report.get(tail).unwrap();

    let isAscending = headValue < tailValue;
    loop {
        if isAscending != (headValue < tailValue) {
            return false;
        }
        let delta = (headValue - tailValue).abs();
        if delta > ALLOWABLE_DIFFERENCE || delta == 0 {
            return false;
        }
        head += 1;
        tail += 1;
        headValue = report.get(head).unwrap();
        tailValue = match report.get(tail) {
            Some(value) => value,
            None => { break; }
        };
    }
    true
}

fn checkIfValidWithSafety(originalReport: &Vec<i32>) -> bool {
    println!("Original Report: {:?}", originalReport);
    let mut report = originalReport.clone();
    let mut head = 0;
    let mut tail = 1;

    let mut safety_triggered = false;

    let mut head_value = report[head];
    let mut tail_value = report[tail];

    let is_ascending = head_value < tail_value;
    loop {
        println!("Head: {} - Tail: {}", head_value, tail_value);
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

fn handle_safety_trigger(safety_triggered: &mut bool, report: &mut Vec<i32>, tail: &usize, tailValue: &mut i32) -> SafetyTriggerResult {
    if (*safety_triggered) {
        println!("Safety Triggered - termination");
        Terminate
    } else {
        println!("Safety Triggered");
        *safety_triggered = true;
        report.remove(*tail);
        *tailValue = match report.get(*tail) {
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