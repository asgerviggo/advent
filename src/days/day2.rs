use itertools::Itertools;

use crate::ImplementPart;
use crate::days::{Part1, Part2};

pub struct Day2;
ImplementPart!(Day2, Part1, part1, create(part1), usize);
ImplementPart!(Day2, Part2, part2, create(part2), usize);

fn create<F>(part: F) -> impl FnOnce(&str) -> usize
where
    F: Fn(usize) -> usize,
{
    move |content: &str| {
        let mut counter = 0;
        for interval in content.split(",") {
            let (startstr, endstr) = interval
                .split_once("-")
                .unwrap();
            // println!("start - end: {:?} - {:?}", startstr, endstr);
            let (start, end) = (
                startstr
                    .parse::<usize>()
                    .unwrap(),
                endstr
                    .trim()
                    .parse::<usize>()
                    .unwrap(),
            );
            for n in start..end + 1 {
                counter += part(n);
            }
        }
        counter
    }
}

fn part1(n: usize) -> usize {
    let str = n.to_string();
    let (left, right) = str.split_at(str.len() / 2);
    if left == right {
        return n;
    }
    return 0;
}

fn part2(n: usize) -> usize {
    let str = n.to_string();
    let length = str.len();

    let factors = factorize(length);

    for factor in factors {
        //let splits = recursplit(str, n);

        let (first, rest) = str.split_at(factor);
        let hit = rest
            .chars()
            .chunks(factor)
            .into_iter()
            .fold(true, |acc, chars| {
                let substr =
                    chars.fold("".to_owned(), |acc, c| acc + &c.to_string());
                if substr == first {
                    acc & true
                } else {
                    acc & false
                }
            });

        if hit {
            // println!("{n}");
            return n;
        }
    }
    0
}

fn factorize(n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for f in 1..n {
        if n % f == 0 {
            out.push(f);
        }
    }
    out
}
