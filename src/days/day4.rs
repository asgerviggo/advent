use std::iter::once;

use crate::days::Solution;

pub struct Day4;
impl Solution for Day4 {
    const VAL: usize = 4;
    type Output = usize;
    fn part1(_: &str) -> Option<Self::Output> {
        None
    }
    fn part2(content: &str) -> Option<Self::Output> {
        Some(run(content))
    }
}

pub fn run(content: &str) -> usize {
    let center_lines: Vec<Vec<bool>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c == '@')
                .collect()
        })
        .collect();

    let floorplan = add_padding(center_lines);

    let initial_count = count_rolls(&floorplan);
    let mut removed = floorplan;
    let mut prev = initial_count;
    let mut cur = prev - 1;
    while cur < prev {
        prev = cur;
        removed = remove_paper(&removed);
        cur = count_rolls(&removed);
    }
    initial_count - cur
}

fn remove_paper(floorplan: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // let mut counter: usize = 0;
    // for triple in floorplan.windows(3) {
    //     let [top, middle, bottom] = triple else {
    //         panic!("Too few lines!")
    //     };

    //     for square in top
    //         .windows(3)
    //         .zip(middle.windows(3))
    //         .zip(bottom.windows(3))
    //     {
    //         let ((top_slice, middle_slice), bottom_slice) = square;
    //         let sum = sum(top_slice) + sum(middle_slice) + sum(bottom_slice);
    //         if middle_slice[1] && sum < 4 {
    //             counter += 1;
    //         }
    //     }
    // }
    // counter

    let new_center = floorplan
        .windows(3)
        .fold(Vec::new(), |mut lines, triple| {
            let [top, middle, bottom] = triple else {
                panic!("Too few lines!")
            };
            let squares = top
                .windows(3)
                .zip(middle.windows(3))
                .zip(bottom.windows(3));
            let new_line = squares.fold(
                Vec::new(),
                |mut line, ((top_slice, middle_slice), bottom_slice)| {
                    let sum =
                        sum(top_slice) + sum(middle_slice) + sum(bottom_slice);
                    line.push(middle_slice[1] && sum > 4);
                    line
                },
            );
            lines.push(new_line);
            lines
        });

    add_padding(new_center)
}

fn add_padding(floorplan: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let first = floorplan
        .first()
        .unwrap();
    let line_length = first.len();
    let empty_line = vec![false; line_length + 2];

    let side_padded = floorplan
        .iter()
        .map(|line| {
            once(false)
                .chain(
                    line.iter()
                        .map(|b| *b),
                )
                .chain(once(false))
                .collect()
        });

    once(empty_line.clone())
        .chain(side_padded)
        .chain(once(empty_line.clone()))
        .collect()
}

fn count_rolls(floorplan: &Vec<Vec<bool>>) -> usize {
    floorplan
        .iter()
        .fold(0, |acc, line| acc + sum(line))
}

fn sum(slice: &[bool]) -> usize {
    slice
        .iter()
        .fold(0, |acc, e| if *e { acc + 1 } else { acc })
}
