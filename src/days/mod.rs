use colored::{ColoredString, Colorize};
use std::cmp::Ordering;
use std::fmt::Display;
use std::time::{Duration, Instant};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub trait Solution {
    const VAL: usize = 0;
    type Output: Display;
    fn part1(_content: &str) -> Option<Self::Output> {
        None
    }
    fn part2(_content: &str) -> Option<Self::Output> {
        None
    }
}

pub fn solve<S: Solution>(content: &str) -> Vec<String> {
    Vec::from([
        format!("part 1 = {}", solve_part1::<S>(content)),
        format!("part 2 = {}", solve_part2::<S>(content)),
    ])
}

pub fn solve_with_time<S: Solution>(content: &str) -> Vec<String> {
    let (result1, time1) = time()(solve_part1::<S>, content, 100);
    let (result2, time2) = time()(solve_part2::<S>, content, 100);
    Vec::from([
        format!("part 1 = {}", result1),
        format!("time 1 = {}", time1),
        format!("part 2 = {}", result2),
        format!("time 2 = {}", time2),
    ])
}

fn time<T>() -> impl FnOnce(T, &str, u64) -> (String, ColoredString)
where
    T: FnOnce(&str) -> String,
{
    |func, content, warn| {
        let begin = Instant::now();
        let result = func(content);
        let end = Instant::now();

        let done_time_val = end - begin;
        let done_time_str =
            match done_time_val.cmp(&Duration::from_millis(warn)) {
                Ordering::Greater => format!("{done_time_val:?}").red(),
                _ => format!("{done_time_val:?}").bold(),
            };
        (result, done_time_str)
    }
}

pub fn solve_part1<S: Solution>(content: &str) -> String {
    match S::part1(content) {
        None => "Not implemented".to_string(),
        Some(sol) => sol.to_string(),
    }
}
pub fn solve_part2<S: Solution>(content: &str) -> String {
    match S::part2(content) {
        None => "Not implemented".to_string(),
        Some(sol) => sol.to_string(),
    }
}

// pub trait Test {
//     type Output: Display;
//     fn test(content: &str) -> Option<Self::Output>;
// }
// pub fn test_day<S: Test>(content: &str, expect: S::Output) {
//     match S::test(content) {
//         Some(output) => {
//             format!("Day {S:2} test: {output}, expected: {expect}")
//         }
//         None => format!("No test available for day {day.val:2}"),
//     }
// }
