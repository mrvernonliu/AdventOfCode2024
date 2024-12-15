use std::env;
use env_logger::Env;
use log::info;

mod days;
mod utils;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    for &day in &get_days_to_run() {
        match day {
            1 => {
                info!("Advent of Code 2024!");
                info!("Day 1:");
                days::day_1::part_1();
                days::day_1::part_2();
            }
            2 => {
                info!("Day 2:");
                days::day_2::part_1();
                days::day_2::part_2();
            }
            3 => {
                info!("Day 3:");
                days::day_3::part_1();
                days::day_3::part_2();
            }
            4 => {
                info!("Day 4:");
                days::day_4::part_1();
                days::day_4::part_2();
            }
            5 => {
                info!("Day 5:");
                days::day_5::part_1_and_2();
            }
            6 => {
                info!("Day 6:");
                days::day_6::part_1();
                days::day_6::part_2();
            }
            7 => {
                info!("Day 7:");
                days::day_7::part_1();
                days::day_7::part_2();
            }
            8 => {
                info!("Day 8:");
                days::day_8::part_1();
                days::day_8::part_2();
            }
            9 => {
                info!("Day 9:");
                days::day_9::part_1();
                days::day_9::part_2();
            }
            10 => {
                info!("Day 10:");
                days::day_10::part_1();
                days::day_10::part_2();
            }
            11 => {
                info!("Day 11:");
                days::day_11::part_1();
                days::day_11::part_2();
            }
            _ => {}
        }
    }
}

fn get_days_to_run() -> Vec<i32> {
    let args: Vec<i32> = env::args().skip(1).filter_map(|arg| arg.parse().ok()).collect();

    match args.as_slice() {
        [start, end] => (*start..=*end).collect(),
        [start] => vec![*start],
        _ => (1..=25).collect(),
    }
}