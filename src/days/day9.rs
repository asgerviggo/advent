use std::collections::HashSet;
use std::iter::once;

use itertools::Itertools;

use crate::ImplementPart;
use crate::days::{NoPart1, Part2};
use crate::util::parse;

pub struct Day9;
impl NoPart1 for Day9 {}
// ImplementPart!(Day9, Part1, part1, run, usize);
ImplementPart!(Day9, Part2, part2, run, usize);

pub fn run(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(",")
                .unwrap();
            (parse::<isize>(x), parse::<isize>(y))
        });

    let squares = tiles
        .clone()
        .enumerate()
        .map(|(index1, (x1, y1))| {
            tiles
                .clone()
                .skip(index1 + 1)
                .map(move |(x2, y2)| {
                    (
                        (x1, y1),
                        (x2, y2),
                        (isize::abs_diff(x2, x1) + 1)
                            * (isize::abs_diff(y2, y1) + 1),
                    )
                })
        })
        .flatten()
        .sorted_by(|(_, _, size_a), (_, _, size_b)| size_b.cmp(size_a));

    let first_tile = tiles
        .clone()
        .take(1)
        .next()
        .unwrap();

    let pairs = tiles
        .clone()
        .zip(
            tiles
                .skip(1)
                .chain(once(first_tile)),
        );

    let circumference =
        pairs.fold(HashSet::new(), |mut acc, ((x1, y1), (x2, y2))| {
            if x1 == x2 {
                let sign = (y1 - y2) / (isize::abs(y2 - y1));
                for y in y1..y2 {
                    acc.remove(&(x1, y));
                    acc.insert((x1 + sign, y));
                }
            }
            if y1 == y2 {
                let sign = (x1 - x2) / (isize::abs(x2 - x1));
                for x in x1..x2 {
                    acc.remove(&(x, y1));
                    acc.insert((x, y1 + sign));
                }
            }
            acc
        });

    for (corner1, corner2, size) in squares {
        let mut collision = false;
        for point in circumference.iter() {
            collision = check_collision(point, (corner1, corner2));
            if collision {
                break;
            }
        }
        if !collision {
            return size as usize;
        }
    }

    // part 1
    // let (_, _, size) = squares
    //     .next()
    //     .unwrap();
    //size as usize
    0
}
fn check_collision(
    (x, y): &(isize, isize),
    ((x1, y1), (x2, y2)): ((isize, isize), (isize, isize)),
) -> bool {
    let min_x = isize::min(x1, x2);
    let max_x = isize::max(x1, x2);
    let min_y = isize::min(y1, y2);
    let max_y = isize::max(y1, y2);
    (x >= &min_x && x <= &max_x) && (y >= &min_y && y <= &max_y)
}
