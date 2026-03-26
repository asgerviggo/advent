use std::fmt::Display;

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

pub trait Part1 {
    type Output: Display;
    fn part1(content: &str) -> Option<Self::Output>;
}
pub trait Part2 {
    type Output: Display;
    fn part2(content: &str) -> Option<Self::Output>;
}

// TODO make this cooler
pub fn solve<S: Part1 + Part2>(content: &str) -> String {
    match (S::part1(content), S::part2(content)) {
        (None, None) => "Not implemented".to_string(),
        (Some(sol1), None) => format!(":::: part 1 = {sol1}"),
        (None, Some(sol2)) => format!(":::: part 2 = {sol2}"),
        (Some(sol1), Some(sol2)) => {
            format!(":::: part 1 = {sol1}\n :::: part 2 = {sol2}")
        }
    }
}
#[allow(dead_code)]
pub fn solve_part1<S: Part1>(content: &str) -> String {
    match S::part1(content) {
        None => "Not implemented".to_string(),
        Some(sol) => format!(":::: part 1 = {sol}"),
    }
}
#[allow(dead_code)]
pub fn solve_part2<S: Part2>(content: &str) -> String {
    match S::part2(content) {
        None => "Not implemented".to_string(),
        Some(sol) => format!(":::: part 2 = {sol}"),
    }
}

pub trait NoPart1 {}
pub trait NoPart2 {}

impl<Day> Part1 for Day
where
    Day: NoPart1,
{
    type Output = usize;
    fn part1(_: &str) -> Option<Self::Output> {
        None
    }
}
impl<Day> Part2 for Day
where
    Day: NoPart2,
{
    type Output = usize;
    fn part2(_: &str) -> Option<Self::Output> {
        None
    }
}
#[macro_export]
macro_rules! ImplementPart {
    ($day: ident, $struct_name: ident, $part: ident, $result: expr, $output: ty) => {
        impl $struct_name for $day {
            type Output = $output;

            fn $part(content: &str) -> Option<Self::Output> {
                Some($result(content))
            }
        }
    };
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
