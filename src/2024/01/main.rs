use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct File {
    left: Vec<u64>,
    right: Vec<u64>,
}
impl File {
    fn parse(s: &str) -> nom::IResult<&str, Self> {
        use nom::{
            bytes::complete::tag,
            character::complete::space1,
            combinator::{eof, map},
            multi::separated_list0,
            sequence::{separated_pair, terminated},
        };

        terminated(
            map(
                separated_list0(
                    tag("\n"),
                    separated_pair(aoc::parse::decimal, space1, aoc::parse::decimal),
                ),
                |pairs| File {
                    left: pairs.iter().map(|(x, _)| x).cloned().collect_vec(),
                    right: pairs.iter().map(|(_, y)| y).cloned().collect_vec(),
                },
            ),
            eof,
        )(&s)
    }
}

fn main() {
    let datastr = std::fs::read_to_string("p1.in.txt").unwrap();
    let (_, file) = File::parse(&datastr).unwrap();
    println!("file = {file:?}");

    {
        println!("-------------- part 1 -------------");
        let (mut left, mut right) = (file.left.clone(), file.right.clone());

        left.sort();
        println!("sorted left {left:?}");
        right.sort();
        println!("sorted right {right:?}");

        let pairs = left.iter().zip(right.iter()).collect::<Vec<_>>();
        println!("pairs {pairs:?}");
        let dists = pairs
            .iter()
            .map(|(&l, &r)| l.abs_diff(r))
            .collect::<Vec<_>>();
        println!("dists {dists:?}");

        let total: u64 = dists.iter().sum();
        println!("total {total:?}");
    }

    {
        println!("-------------- part 2 -------------");
        let (left, right) = (file.left.clone(), file.right.clone());
        println!("left {left:?}");
        println!("right {right:?}");

        let right_counts = right.iter().fold(HashMap::new(), |mut acc, &r| {
            *acc.entry(r).or_insert(0) += 1;
            acc
        });

        let similarities = left
            .iter()
            .map(|l| l * *right_counts.get(l).unwrap_or(&0))
            .collect::<Vec<_>>();
        println!("similarities {similarities:?}");

        let total: u64 = similarities.iter().cloned().sum();
        println!("total {total:?}");
    }
}
