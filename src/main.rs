use std::{env, path::PathBuf};

pub mod day1;
pub mod day2;
pub mod day3;

pub trait AdventOfCode {
    fn run(&mut self, base_dir: &PathBuf);
}

fn main() {
    println!("Hello, world!");

    let base_dir: PathBuf = env::current_dir()
        .expect("failed to get current dir")
        .join("input/2021");
    // println!("{}", &base_dir.to_string_lossy());

    // setup days
    let days: Vec<Box<dyn AdventOfCode>> = vec![
        Box::new(day1::Data::default()),
        Box::new(day2::Data::default()),
        Box::new(day3::Data::default()),
    ];

    for mut day in days {
        day.run(&base_dir);
    }
}


// use aoc_runner_derive;
// use aoc_runner;

// use aoc_runner_derive::aoc_main;

// aoc_main! { lib = advent_of_code_2015 }