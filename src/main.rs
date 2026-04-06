use clap::Parser;
use colored::Colorize;
use std::fs::read_to_string;
use std::time::Instant;

mod days;
mod util;

use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::days::day7::Day7;
use crate::days::day8::Day8;
use crate::days::day9::Day9;
use crate::days::day10::Day10;
use crate::days::day11::Day11;
use crate::days::day12::Day12;

use crate::days::{Solution, solve, solve_with_time};
use crate::util::parse;

#[derive(Parser)]
struct Arguments {
    #[clap(long, short, action)]
    time: bool,

    #[clap(long, short, default_value = "1-12")]
    range: String,
}

fn run<T: Solution>(solver: fn(&str) -> Vec<String>, path: &str) {
    let content = read_to_string(path).expect("could not read file");
    let result = solver(content.as_str());

    println!("Day {}", T::VAL);
    for line in result {
        let spacer = " :::: ".truecolor(0, 255, 136);
        println!("{spacer}{line}");
    }
}

fn main() {
    let Arguments { time, range } = Arguments::parse();
    let days_to_run: Vec<usize> = range
        .split(",")
        .map(|range| match range.split_once("-") {
            Some((min, max)) => parse(min)..(parse::<usize>(max) + 1),
            None => parse(range)..(parse::<usize>(range) + 1),
        })
        .flatten()
        .collect();

    macro_rules! runner {
        ($day_struct: ident) => {
            let day = $day_struct::VAL;
            let solver = match time {
                true => solve_with_time::<$day_struct>,
                false => solve::<$day_struct>,
            };
            if days_to_run.contains(&day) {
                run::<$day_struct>(
                    solver,
                    format!("../data/days/day{day}.txt").as_str(),
                );
            }
        };
    }

    let begin_time = Instant::now();

    runner!(Day1);
    runner!(Day2);
    runner!(Day3);
    runner!(Day4);
    runner!(Day5);
    runner!(Day6);
    runner!(Day7);
    runner!(Day8);
    runner!(Day9);
    runner!(Day10);
    runner!(Day11);
    runner!(Day12);

    let final_time = Instant::now();
    println!("\nTotal time: {:#?}", final_time - begin_time);

    // if args.test {
    //     todo!()
    //     // let test1 = test::<Day1>(include_str!("../data/days/day1.txt"), 32491);
    // }
}
