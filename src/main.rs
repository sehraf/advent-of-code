use std::{env, path::PathBuf};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        Box::new(day11::Data::default()),
        Box::new(day12::Data::default()),
        Box::new(day13::Data::default()),
        Box::new(day14::Data::default()),
        Box::new(day15::Data::default()),
    ];

    for mut day in days {
        day.run(&base_dir);
    }
}
