// use std::iter::Zip;

use crate::util::parse;

use crate::ImplementPart;
use crate::days::{Part1, Part2};

pub struct Day6;
ImplementPart!(Day6, Part1, part1, part1, usize);
ImplementPart!(Day6, Part2, part2, run, usize);

#[allow(unused_macros)]
macro_rules! split {
    ($($x: expr),*) => {
        let () $(
            $x.next().unwrap().chars()
        )*
    };
}

// macro_rules! zip {
//     ($x: expr) => ($x);
//     ($x: expr, $($y: expr), +) => (
//         $x.iter().zip(
//             zip!($($y), +)
//         ).flatten()
//     )
// }

// pub fn run(input: &str) -> usize {
//     let (numbers1, numbers2, numbers3, numbers4, operators) =
//         split!(input, '\n');

//     rows.fold(init.chars(), |acc, row| acc.zip(row.chars()));

//     let zipped = zip!(numbers1, numbers2, numbers3, numbers4);
// }
//

pub fn run(input: &str) -> usize {
    let (rest, operators) = input
        .trim()
        .rsplit_once('\n')
        .unwrap();
    let (numbers1, rest) = rest
        .split_once('\n')
        .unwrap();
    let (numbers2, rest) = rest
        .split_once('\n')
        .unwrap();
    let (numbers3, rest) = rest
        .split_once('\n')
        .unwrap();
    let numbers4 = rest;

    let mut prestate = Vec::new();
    prestate.push(Vec::new());
    let to_operate_on = numbers1
        .chars()
        .zip(numbers2.chars())
        .zip(numbers3.chars())
        .zip(numbers4.chars())
        .fold(prestate, |mut acc, (((n1, n2), n3), n4)| {
            let strn = format!("{n1}{n2}{n3}{n4}");
            let trimmed = strn.trim();
            if trimmed.is_empty() {
                acc.push(Vec::new());
            } else {
                let mut numbers = acc
                    .pop()
                    .unwrap();

                numbers.push(parse::<usize>(trimmed));
                acc.push(numbers)
            }
            acc
        });

    to_operate_on
        .iter()
        .zip(operators.split_whitespace())
        .fold(0, |acc, (num_vec, operator)| {
            acc + match operator {
                "*" => num_vec
                    .iter()
                    .fold(1, |prod, number| prod * number),
                "+" => num_vec
                    .iter()
                    .fold(0, |prod, number| prod + number),
                _ => panic!("Not an operator"),
            }
        })
}

pub fn part1(input: &str) -> usize {
    let mut numbers = input.split("\n");
    //.map(|list| list.split(" ").map(|val| parse::<usize>(val)));

    let numbers1 = numbers
        .next()
        .unwrap();
    let numbers2 = numbers
        .next()
        .unwrap();
    let numbers3 = numbers
        .next()
        .unwrap();
    let numbers4 = numbers
        .next()
        .unwrap();
    let operators = numbers
        .next()
        .unwrap();

    let zipped = operators
        .split_whitespace()
        .zip(
            numbers1
                .split_whitespace()
                .map(|val| parse::<usize>(val)),
        )
        .zip(
            numbers2
                .split_whitespace()
                .map(|val| parse::<usize>(val)),
        )
        .zip(
            numbers3
                .split_whitespace()
                .map(|val| parse::<usize>(val)),
        )
        .zip(
            numbers4
                .split_whitespace()
                .map(|val| parse::<usize>(val)),
        );

    zipped.fold(0, |acc, val| match val {
        (((("*", num1), num2), num3), num4) => acc + num1 * num2 * num3 * num4,
        (((("+", num1), num2), num3), num4) => acc + num1 + num2 + num3 + num4,
        ((((op, num1), num2), num3), num4) => panic!(
            "Operator: {}, Num1: {}, Num2: {}, Num3: {}, Num4: {}",
            op, num1, num2, num3, num4
        ),
    })
}
