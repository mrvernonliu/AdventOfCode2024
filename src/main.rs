use env_logger::Env;
use log::info;

mod days;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Advent of Code 2024!");
    info!("Day 1:");
    days::day_1::part_1();
    days::day_1::part_2();

    info!("Day 2:");
    days::day_2::part_1();
    days::day_2::part_2();

    info!("Day 3:");
    days::day_3::part_1();
    days::day_3::part_2();

    info!("Day 4:");
    days::day_4::part_1();
    days::day_4::part_2();

    info!("Day 5:");
    days::day_5::part_1_and_2();

}