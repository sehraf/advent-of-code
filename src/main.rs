use std::{env, path::PathBuf, time::Instant};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub trait AdventOfCode {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64);
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
        Box::new(day16::Data::default()),
        Box::new(day17::Data::default()),
        Box::new(day18::Data::default()),
        Box::new(day19::Data::default()),
        Box::new(day20::Data::default()),
        Box::new(day21::Data::default()),
        Box::new(day22::Data::default()),
        Box::new(day23::Data::default()), // the unloved child nobody talks about
        Box::new(day24::Data::default()),
    ];

    let mut results = vec![];
    for mut day in days {
        let t0 = Instant::now();
        let ret = day.run(&base_dir);
        results.push((ret, Instant::now().duration_since(t0)));
    }

    println!();
    println!("Duration summary");
    for i in 0..results.len() {
        let duration = format!("{:?}", results[i].1);

        println!(
            "Day {:2}: 1) {:17} 2) {:17} - took {:>15}",
            i + 1,
            results[i].0 .0,
            results[i].0 .1,
            duration
        );
    }
}
