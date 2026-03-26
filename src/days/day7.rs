use crate::ImplementPart;
use crate::days::{Part1, Part2};

pub struct Day7;
ImplementPart!(Day7, Part1, part1, part1, usize);
ImplementPart!(Day7, Part2, part2, run, usize);

pub fn part1(input: &str) -> usize {
    let (first, rest) = input
        .split_once("\n")
        .unwrap();
    let line_length = first.len();
    let state: Vec<bool> = first
        .chars()
        .map(|c| if c == 'S' { true } else { false })
        .collect();
    let (_, amount) = rest
        .lines()
        .fold((state, 0), |(acc_state, mut acc_sum), line| {
            let splitters = line
                .chars()
                .map(|c| if c == '^' { true } else { false });
            //let new_state: Vec<bool> = Vec::new();
            let new_state = acc_state
                .iter()
                .zip(splitters)
                .enumerate()
                .fold(
                    vec![false; line_length],
                    |mut acc, (index, (beam, split))| {
                        if *beam {
                            if split {
                                acc[index - 1] = true;
                                acc[index + 1] = true;
                                acc_sum += 1
                            } else {
                                acc[index] = true;
                            }
                        };
                        acc
                    },
                );
            (new_state, acc_sum)
        });

    amount
}

pub fn run(input: &str) -> usize {
    let (first, rest) = input
        .split_once("\n")
        .unwrap();
    let line_length = first.len();
    let state: Vec<usize> = first
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    let final_state = rest
        .lines()
        .fold(state, |acc_state, line| {
            let splitters = line
                .chars()
                .map(|c| if c == '^' { true } else { false });
            //let new_state: Vec<bool> = Vec::new();
            let new_state = acc_state
                .iter()
                .zip(splitters)
                .enumerate()
                .fold(
                    vec![0; line_length],
                    |mut acc, (index, (beam, split))| {
                        if *beam > 0 {
                            if split {
                                acc[index - 1] += beam;
                                acc[index + 1] += beam;
                            } else {
                                acc[index] += *beam;
                            }
                        };
                        acc
                    },
                );
            // println!("{line}");
            // println!("{:?}", new_state);
            new_state
        });

    final_state
        .iter()
        .fold(0, |acc, beams| acc + beams)
}
