use std::{
    fmt::{Debug, Display},
    iter::once,
    ops::{Add, BitXor, Div, Mul, Sub},
    str::FromStr,
};

use itertools::Itertools;

use crate::days::Part1;
use crate::util::parse;
use crate::{ImplementPart, days::NoPart2};

pub struct Day10;
ImplementPart!(Day10, Part1, part1, part1, usize);
impl NoPart2 for Day10 {}
// ImplementPart!(Day10, Part2, part2, part2, i16);

fn trim(string: &str) -> &str {
    string.trim_matches(&['{', '}', '(', ')', '[', ']'][..])
}

macro_rules! create_operator_impl {
    ($impl_name: ident, $method_name: ident, $closure: expr) => {
        impl<T> $impl_name for Button<T>
        where
            T: Copy + $impl_name<Output = T>,
        {
            fn $method_name(self, rhs: Self) -> Self {
                Button(
                    self.0
                        .iter()
                        .zip(
                            rhs.0
                                .iter(),
                        )
                        .map($closure)
                        .collect(),
                )
            }
            type Output = Self;
        }
    };
}

#[derive(Debug, Clone)]
struct Button<T>(Vec<T>);

impl<T: Clone> Button<T> {
    fn new() -> Button<T> {
        Button(Vec::new())
    }
    fn push(&mut self, item: T) {
        Vec::push(
            self.0
                .as_mut(),
            item,
        )
    }
}

create_operator_impl!(Add, add, |(&a, &b)| a + b);
create_operator_impl!(Sub, sub, |(&a, &b)| a - b);
create_operator_impl!(Mul, mul, |(&a, &b)| a * b);
create_operator_impl!(Div, div, |(&a, &b)| a / b);
create_operator_impl!(BitXor, bitxor, |(&a, &b)| a ^ b);

fn create_iterator<'a>(
    input: &'a str,
) -> impl Iterator<Item = (&'a str, &'a str, &'a str)> {
    input
        .lines()
        .map(|line: &str| {
            line.split_once(" ")
                .unwrap()
        })
        .map(|(indicator, rest): (&str, &str)| {
            let (buttons, joltage) = rest
                .rsplit_once(" ")
                .unwrap();
            (trim(indicator), buttons, trim(joltage))
        })
}

fn init_buttons<T>(
    buttons: &str,
    size: usize,
    fill: T,
    null: T,
) -> Vec<Button<T>>
where
    T: Clone,
{
    buttons
        .split(" ")
        .map(|button| {
            Button(
                trim(button)
                    .split(",")
                    .fold(vec![null.clone(); size], |mut acc, i| {
                        let index = parse::<usize>(i);
                        acc[index] = fill.clone();
                        acc
                    }),
            )
        })
        .collect()
}
fn init_indicator(target: &str) -> Button<bool> {
    Button(
        trim(target)
            .chars()
            .map(|c| if c == '#' { true } else { false })
            .collect(),
    )
}

fn part1(input: &str) -> usize {
    let split = create_iterator(input);
    let mut counter = 0;
    for (indicator, buttons, _) in split {
        let buttons_ints =
            init_buttons::<bool>(buttons, indicator.len(), true, false);

        let indicator = init_indicator(indicator);
        let amount = find_min_pushes(buttons_ints, indicator, naive);

        counter += amount;
    }
    counter
}

fn init_joltage<T: FromStr>(target: &str) -> Button<T> {
    Button(
        trim(target)
            .split(",")
            .map(parse)
            .collect(),
    )
}

#[allow(dead_code)]
fn part2(input: &str) -> i16 {
    let split = create_iterator(input);
    let mut counter = 0;
    for (_, buttons, joltage) in split {
        let buttons_ints = init_buttons::<i16>(buttons, joltage.len(), 1, 0);

        let joltage = init_joltage::<i16>(joltage);
        let amount = find_min_pushes(buttons_ints, joltage, linalg_finder);

        counter += amount;
    }
    counter
}

fn find_min_pushes<T, D>(
    buttons: Vec<Button<T>>,
    target: Button<T>,
    tester: fn(Vec<&Button<T>>, &Button<T>) -> Option<D>,
) -> D
where
    T: Debug,
    D: Display,
{
    let size = buttons.len();
    for n in 1..size + 1 {
        for combination in buttons
            .iter()
            .combinations(n)
        {
            match tester(combination, &target) {
                Some(amount) => return amount,
                None => (),
            }
        }
    }
    panic!("not found");
}

fn naive(
    combination: Vec<&Button<bool>>,
    indicator: &Button<bool>,
) -> Option<usize> {
    let Button(test) = combination
        .iter()
        .cloned()
        .fold(indicator.clone(), |acc, button| acc ^ button.clone());

    test.iter()
        .fold(true, |acc, ind| acc && !*ind)
        .then_some(combination.len())
}

fn linalg_finder(
    combination: Vec<&Button<i16>>,
    joltage: &Button<i16>,
) -> Option<i16> {
    let col_size = joltage
        .0
        .len();
    // combination.push(joltage);
    let row_size = combination.len() - 1;

    // println!("{combination:?}");
    let rows = transpose(combination, col_size);
    let upper = upper_triangular(rows, row_size);
    //println!("{upper:?}");

    // let cols = transpose(upper, row_size);
    let reduced = diagonal(upper);

    // check for empty rows with non-zero joltage

    let Button(final_joltage) = reduced
        .last()
        .unwrap();

    // Check for negative joltage
    let sum = final_joltage
        .iter()
        .fold(0, |acc, e| acc + *e);

    Some(sum)
}

fn transpose(cols: Vec<&Button<i16>>, col_size: usize) -> Vec<Button<i16>> {
    let mut rows: Vec<Button<i16>> = vec![Button::new(); col_size];
    for Button(button) in cols.iter() {
        for (i, &b) in button
            .iter()
            .enumerate()
        {
            rows[i].push(b);
        }
    }
    rows
}

fn gaussian_pass(
    rows: Vec<&Button<i16>>,
    current: Button<i16>,
) -> Vec<Button<i16>> {
    let index: usize = current
        .0
        .iter()
        .find_position(|v| **v != 0)
        .and_then(|(i, _)| Some(i))
        .expect("Row shouldn't be empty");
    let to_sub = rows
        .iter()
        .filter(|Button(b)| b[index] != 0);
    let rest = rows
        .iter()
        .filter(|Button(b)| b[index] == 0)
        .map(|b| b.to_owned());
    to_sub
        .map(|&row| {
            if row.0[index] == 1 {
                row.to_owned() - current.clone()
            } else {
                row.to_owned()
            }
        })
        .chain(rest.cloned())
        .collect()
}

fn upper_triangular(
    rows: Vec<Button<i16>>,
    row_size: usize,
) -> Vec<Button<i16>> {
    let mut sorted = rows
        .iter()
        .sorted_by_key(|Button(b)| b[row_size]);
    let lowest = sorted.next();
    if lowest.is_some() {
        let low = lowest.unwrap();
        let remaining = gaussian_pass(sorted.collect(), low.clone());
        once(low.clone())
            .chain(upper_triangular(remaining, row_size).into_iter())
            .collect()
    } else {
        rows
    }
}
fn diagonal(rows: Vec<Button<i16>>) -> Vec<Button<i16>> {
    rows
}

// fn remove_zero_jolts(
//     mut buttons: Vec<&Button<i16>>,
//     Button(joltage): &Button<i16>,
// ) -> Vec<&Button<i16>> {
//     let indices = joltage
//         .iter()
//         .enumerate()
//         .filter(|(_, jolt)| **jolt == 0);

//     let size = joltage.len();

//     buttons.retain(|Button(button)| {
//         indices
//             .clone()
//             .fold(true, |acc, (i, _)| acc && button[i] == 0)
//     });
//     buttons
// }

// fn remove_uniquely_determined_row(
//     mut buttons: Vec<&Button<i16>>,
//     Button(joltage): &Button<i16>,
// ) -> Button<i16> {
//     let Button(sum_vec) = buttons
//         .iter()
//         .cloned()
//         .fold(Button::new(0, size), |acc, button| acc + *button);

//     // println!("---");
//     // println!("Joltage: {:?}", joltage);
//     // buttons
//     //     .iter()
//     //     .for_each(|b| println!("{:?}", b.0));
//     // println!("Sum_vec: {sum_vec:?}");

//     let ((min_index, min), (max_index, max)) = sum_vec
//         .iter()
//         .enumerate()
//         .filter(|(_, e)| **e > 0)
//         .minmax_by_key(|(_, e)| **e)
//         .into_option()?;

//     if *min == 1 {
//         let &amount = joltage
//             .get(min_index)
//             .unwrap();
//         let button = buttons
//             .iter()
//             .find(|Button(b)| b[min_index] == 1)
//             .cloned()
//             .expect("This exists");

//         // Check overflow
//         Button(*joltage) - button * Button::new(amount, size)
//     } else {
//         Button(*joltage)
//     }
// }

// println!("Min - Max: {min_index} - {max_index}");

// println!("---");

// let buttons_amount = buttons.len();
// let sum = joltage
//     .iter()
//     .fold(0, |acc, jolt| acc + jolt);
// if (*max).into() == buttons_amount {
//     println!("here: {sum}");
//     match sum {
//         0 => return Some(joltage[max_index].into()),
//         _ => return None,
//     }
// }

// let (index, low_jolt) = joltage
//     .iter()
//     .enumerate()
//     .filter(|(_, e)| **e > 0)
//     .min_by_key(|(_, e)| *e)
//     .expect("Shouldn't be empty");

// let mut jolt = *low_jolt;

// let lowest = buttons
//     .iter()
//     .filter(|Button(b)| b[index] == 1);
// let lowest_length = lowest
//     .clone()
//     .count();

// //println!("Lowest: {:?}", lowest);
// // return error here if lowest is empty

// let weights = lowest
//     .clone()
//     .enumerate()
//     .map(|(i, _)| {
//         if i == lowest_length {
//             jolt + 1
//         } else {
//             let r = rand::random_range(0..jolt + 1);
//             jolt -= r;
//             r
//         }
//     });

// let new_joltage = lowest
//     .cloned()
//     .zip(weights)
//     .fold(Button(joltage.clone()), |acc, (low, weight)| {
//         // Check overflow
//         acc - low * Button::new(weight, size)
//     });

// println!("New joltage: {:?}", new_joltage);
// if joltage
//     .iter()
//     .zip(
//         new_joltage
//             .clone()
//             .0,
//     )
//     .fold(0, |acc, (old, new)| acc + old - new)
//     == 0
// {
//     panic!("stuck!")
// }

// *low_jolt as usize + part2(buttons, new_joltage)
//     }
// }
