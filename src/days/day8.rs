use crate::util::parse;
use std::collections::HashSet;

use crate::ImplementPart;
use crate::days::{NoPart1, Part2};

pub struct Day8;
impl NoPart1 for Day8 {}
// ImplementPart!(Day8, Part1, part1, run, usize);
ImplementPart!(Day8, Part2, part2, run, usize);

fn distance(
    (x1, y1, z1): (isize, isize, isize),
    (x2, y2, z2): (isize, isize, isize),
) -> isize {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2)
}

fn print_tuple((x, y, z): (isize, isize, isize)) -> String {
    format!("{x},{y},{z}")
}

pub fn run(input: &str) -> usize {
    let points = input
        .split("\n")
        .take_while(|line| line.len() > 0)
        .map(|line| {
            let (x, rest) = line
                .split_once(",")
                .unwrap();
            let (y, z) = rest
                .split_once(",")
                .unwrap();
            (parse::<isize>(x), parse::<isize>(y), parse::<isize>(z))
        });

    let mut distances: Vec<_> = points
        .clone()
        .enumerate()
        .map(|(index1, point1)| {
            points
                .clone()
                .skip(index1 + 1)
                .map(move |point2| {
                    (
                        print_tuple(point1),
                        print_tuple(point2),
                        distance(point1, point2),
                    )
                })
        })
        .flatten()
        .collect();

    let mut groups = Vec::<HashSet<String>>::new();

    distances.sort_by(|a, b| {
        a.2.cmp(&b.2)
    });

    /*
    for distance in distances.clone().iter().take(1000) {
        println!("{:?}", distance);
    }*/

    for (label1, label2, _) in distances.iter() {
        let new_group =
            HashSet::<String>::from([label1.clone(), label2.clone()]);

        let mut added = HashSet::<usize>::new();
        for (group_index, group) in groups
            .iter_mut()
            .enumerate()
        {
            if group.contains(label1) || group.contains(label2) {
                group.extend(new_group.clone());
                added.insert(group_index);
                if group.len() == 1000 {
                    let (x1, _) = label1
                        .split_once(",")
                        .unwrap();
                    let (x2, _) = label2
                        .split_once(",")
                        .unwrap();
                    return parse::<usize>(x1) * parse::<usize>(x2);
                }
            }
        }
        if added.len() == 2 {
            let mut iter = added.iter();
            let index1 = iter
                .next()
                .unwrap();
            let index2 = iter
                .next()
                .unwrap();
            if index1 != index2 {
                let group1 = groups[*index1].clone();
                groups[*index2].extend(group1);
                if groups[*index2].len() == 1000 {
                    let (x1, _) = label1
                        .split_once(",")
                        .unwrap();
                    let (x2, _) = label2
                        .split_once(",")
                        .unwrap();
                    return parse::<usize>(x1) * parse::<usize>(x2);
                }
                groups.remove(*index1);
            }
        }
        if added.len() == 0 {
            groups.push(new_group);
        }
    }

    groups.sort_by(|a, b| {
        b.len()
            .cmp(&a.len())
    });

    groups
        .iter()
        .take(3)
        .fold(1, |acc, g| acc * g.len())
}
