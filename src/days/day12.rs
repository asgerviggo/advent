use crate::ImplementPart;
use crate::days::{NoPart2, Part1};

pub struct Day12;
ImplementPart!(Day12, Part1, part1, run, usize);
impl NoPart2 for Day12 {}
// ImplementPart!(Day12, Part2, part2, create(part2), usize);

pub fn run(input: &str) -> usize {
    let (shapes, regions) = input
        .rsplit_once("\n\n")
        .unwrap();

    let presents = shapes
        .split("\n\n")
        .map(|present| {
            present
                .split_once(":")
                .unwrap()
                .1
        });

    let areas = regions
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
        })
        .map(|(area, presents)| -> ((usize, usize), Vec<usize>) {
            (
                area.split_once("x")
                    .map(|(x, y)| {
                        (
                            x.parse()
                                .unwrap(),
                            y.parse()
                                .unwrap(),
                        )
                    })
                    .unwrap(),
                presents
                    .trim()
                    .split(" ")
                    .map_while(|p| match p.parse() {
                        Ok(num) => Some(num),
                        Err(_) => None,
                    })
                    .collect(),
            )
        });

    let presents_spaces = presents.map(|present| {
        present
            .matches('#')
            .count()
    });

    //println!("Presents: {:?}, Areas: {:?}", presents, areas);
    let (counter, _) = areas.fold(
        (0, Vec::new()),
        |(amount, mut remaining), ((x, y), quota)| {
            let (presents_amount, spaces_amount) = quota
                .iter()
                .zip(presents_spaces.clone())
                .fold(
                    (0, 0),
                    |(presents_amount, spaces_amount), (presents, spaces)| {
                        (
                            presents_amount + presents,
                            spaces_amount + presents * spaces,
                        )
                    },
                );

            let space = x / 3 * y / 3;
            if presents_amount <= space {
                (amount + 1, remaining)
            } else {
                if spaces_amount <= x * y {
                    remaining.push((x * y, spaces_amount));
                }
                (amount, remaining)
            }
        },
    );
    //println!("Possible: {:?}", remaining);

    counter
}
