use crate::days::Solution;

pub struct Day1;
impl Solution for Day1 {
    const VAL: usize = 1;
    type Output = i32;
    fn part1(content: &str) -> Option<Self::Output> {
        Some(run1(content))
    }
    fn part2(content: &str) -> Option<Self::Output> {
        Some(run2(content))
    }
}

fn run2(content: &str) -> i32 {
    let mut dial: i32 = 50;
    let mut counter: i32 = 0;

    for line in content.lines() {
        let before = dial.clone();
        dial += match line.split_at(1) {
            ("R", amount) => amount
                .parse::<i32>()
                .unwrap(),
            ("L", amount) => -amount
                .parse::<i32>()
                .unwrap(),
            _ => 0,
        };
        let difference = i32::abs(dial - before);

        counter += difference % 100; // + passed_zero;
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
