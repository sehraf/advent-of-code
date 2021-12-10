use std::{env, path::PathBuf};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
mod day10;

pub trait AdventOfCode {
    fn run(&mut self, base_dir: &PathBuf);
}

fn main() {
    println!("Hello, world!");

    let base_dir: PathBuf = env::current_dir()
        .expect("failed to get current dir")
        .join("input/2021");

    // setup days
    let days: Vec<Box<dyn AdventOfCode>> = vec![
        Box::new(day1::Data::default()),
        Box::new(day2::Data::default()),
        Box::new(day3::Data::default()),
        Box::new(day4::Data::default()),
        Box::new(day5::Data::default()),
        Box::new(day6::Data::default()),
        Box::new(day7::Data::default()),
        Box::new(day8::Data::default()),
        Box::new(day9::Data::default()),
        Box::new(day10::Data::default()),
    ];

    for mut day in days {
        day.run(&base_dir);
    }
}
