use crate::ImplementPart;
use crate::days::{NoPart1, Part2};

pub struct Day3;
impl NoPart1 for Day3 {}
// ImplementPart!(Day3, Part1, part1, run, usize);
ImplementPart!(Day3, Part2, part2, run, usize);

pub fn run(content: &str) -> usize {
    let length = 2;
    content
        .lines()
        .fold((0, length), &find_joltage)
        .0
}

fn find_joltage(
    (counter, length): (usize, usize),
    pack: &str,
) -> (usize, usize) {
    let (joltage, _, _) = (0..length)
        .rev()
        .fold((0, pack.len(), pack), &count_voltage);
    (counter + joltage, length)
}

fn count_voltage(
    (joltage, cut_off, pack): (usize, usize, &str),
    n: usize,
) -> (usize, usize, &str) {
    let (index, value) = extract(pack, cut_off, n);
    let n_sized = n
        .try_into()
        .unwrap();
    let power = 10usize.pow(n_sized) * value;
    (joltage + power, index + 1, pack)
}

fn extract(pack: &str, take: usize, skip: usize) -> (usize, usize) {
    pack.chars()
        .rev()
        .skip(skip)
        .take(take)
        .map(|e| {
            e.to_digit(10)
                .unwrap() as usize
        })
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(&b))
        .unwrap()
}
