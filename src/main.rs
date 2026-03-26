use crate::days::day1::Day1;
// use crate::days::day2::Day2;
// use crate::days::day3::Day3;
// use crate::days::day4::Day4;
// use crate::days::day5::Day5;
// use crate::days::day6::Day6;
// use crate::days::day7::Day7;
// use crate::days::day8::Day8;
// use crate::days::day9::Day9;
// use crate::days::day10::Day10;
//use crate::days::day11::Day11;
// use crate::days::day12::Day12;

use crate::days::solve;
use clap::Parser;

mod days;
mod util;

#[derive(Parser)]
struct Arguments {
    //test: bool,
    // range: String,
}

fn main() {
    // let args = Arguments::parse();

    // if args.test {
    //     todo!()
    //     // let test1 = test::<Day1>(include_str!("../data/days/day1.txt"), 32491);
    // }

    let result1 = solve::<Day1>(include_str!("../data/days/day1.txt"))
        .expect("Day 1 not implemented.");
    println!("Day 1 ::: {result1}");
}
