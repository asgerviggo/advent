use crate::ImplementPart;
use crate::days::{Part1, Part2};

pub struct Day1;
ImplementPart!(Day1, Part1, part1, run1, i32);
ImplementPart!(Day1, Part2, part2, run2, i32);

fn run2(content: &str) -> i32 {
    let mut dial: i32 = 50;
    let mut counter: i32 = 0;

    for line in content.lines() {
        let (sign, change) = match line.split_at(1) {
            ("R", amount) => (
                1,
                amount
                    .parse::<i32>()
                    .unwrap(),
            ),
            ("L", amount) => (
                -1,
                amount
                    .parse::<i32>()
                    .unwrap(),
            ),
            _ => (1, 0),
        };
        for _ in 0..change {
            dial += sign;
            let digits = dial
                .to_string()
                .chars()
                .rev()
                .take(2)
                .collect::<String>();
            if dial == 0 || digits == "00" {
                counter += 1;
            }
        }

        //println!(
        //    "line: {:?}, dial: {:?}, dif: {:?}, counter: {:?}",
        //    line, dial, dif, counter
        //);
    }
    counter
}

fn run1(content: &str) -> i32 {
    let mut dial: i32 = 50;
    let mut counter: i32 = 0;

    for line in content.lines() {
        dial += match line.split_at(1) {
            ("R", amount) => amount
                .parse::<i32>()
                .unwrap(),
            ("L", amount) => -amount
                .parse::<i32>()
                .unwrap(),
            _ => 0,
        };
        let digits = dial
            .to_string()
            .chars()
            .rev()
            .take(2)
            .collect::<String>();

        //println!("line: {:?}, dial: {:?}, digits: {:?}", line, dial, digits);

        if dial == 0 || digits == "00" {
            counter += 1;
        }
    }
    counter
}
