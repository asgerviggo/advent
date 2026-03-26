use itertools::Itertools;

use crate::ImplementPart;
use crate::days::{Part1, Part2};

pub struct Day5;
ImplementPart!(Day5, Part1, part1, part1, usize);
ImplementPart!(Day5, Part2, part2, run, usize);

fn parse(val: &str) -> isize {
    val.parse::<isize>()
        .unwrap()
}

fn count(ranges: &str, id: isize) -> usize {
    for range in ranges.lines() {
        let (low, high) = range
            .split_once("-")
            .map(|(l, h)| (parse(l), parse(h)))
            .unwrap();
        if id - low > 0 && id - high < 0 {
            return 1;
        }
    }
    return 0;
}

pub fn part1(input: &str) -> usize {
    let mut counter = 0;
    let (ranges, ids) = input
        .split_once("\n\n")
        .unwrap();

    for id in ids.lines() {
        let val = parse(id);
        counter += count(ranges, val)
    }
    counter
}

pub fn run(input: &str) -> usize {
    let (ranges, _) = input
        .split_once("\n\n")
        .unwrap();

    let ranges: Vec<(isize, isize)> = ranges
        .lines()
        .map(|range| {
            range
                .split_once("-")
                .map(|(l, h)| (parse(l), parse(h)))
                .unwrap()
        })
        .sorted_by(|(a_low, _), (b_low, _)| a_low.cmp(b_low))
        .collect();

    let ranges = build_non_overlapping(ranges);

    count_ids(ranges)
        .try_into()
        .unwrap()
}

fn build_non_overlapping(ranges: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut output = Vec::new();
    for (low, high) in ranges {
        let (last_low, mut last_high) = output
            .pop_if(|(_, last_high)| low <= *last_high)
            .unwrap_or_else(|| (low, high));
        if high > last_high {
            last_high = high;
        }
        output.push((last_low, last_high));
    }
    output
}

fn count_ids(ranges: Vec<(isize, isize)>) -> isize {
    ranges
        .iter()
        .fold(0, |acc, (low, high)| acc + high - low + 1)
}
