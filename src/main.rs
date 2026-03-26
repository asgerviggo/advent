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

use crate::days::solve;
use clap::Parser;
use std::time::Instant;

mod days;
mod util;

#[derive(Parser)]
struct Arguments {
    //test: bool,
    // range: String,
}

macro_rules! runday {
    ($day: literal, $solver: expr, $path: literal) => {
        let begin_time = Instant::now();
        let result = $solver(include_str!($path));
        let next_time = Instant::now();
        println!("Day {} \n {}", $day, result);
        println!(" :::: time: {:#?}", next_time - begin_time);
    };
}

fn main() {
    // let args = Arguments::parse();

    // if args.test {
    //     todo!()
    //     // let test1 = test::<Day1>(include_str!("../data/days/day1.txt"), 32491);
    // }

    let begin_time = Instant::now();
    runday!(1, solve::<Day1>, "../data/days/day1.txt");
    runday!(2, solve::<Day2>, "../data/days/day2.txt");
    runday!(3, solve::<Day3>, "../data/days/day3.txt");
    runday!(4, solve::<Day4>, "../data/days/day4.txt");
    runday!(5, solve::<Day5>, "../data/days/day5.txt");
    runday!(6, solve::<Day6>, "../data/days/day6.txt");
    runday!(7, solve::<Day7>, "../data/days/day7.txt");
    runday!(8, solve::<Day8>, "../data/days/day8.txt");
    runday!(9, solve::<Day9>, "../data/days/day9.txt");
    runday!(10, solve::<Day10>, "../data/days/day10.txt");
    runday!(11, solve::<Day11>, "../data/days/day11.txt");
    runday!(12, solve::<Day12>, "../data/days/day12.txt");
    let final_time = Instant::now();
    println!("\nTotal time: {:#?}", final_time - begin_time);
}
