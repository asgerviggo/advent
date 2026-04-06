use crate::days::Solution;

pub struct Day7;
impl Solution for Day7 {
    const VAL: usize = 7;
    type Output = usize;
    fn part1(content: &str) -> Option<Self::Output> {
        Some(part1(content))
    }
    fn part2(content: &str) -> Option<Self::Output> {
        Some(run(content))
    }
}

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
