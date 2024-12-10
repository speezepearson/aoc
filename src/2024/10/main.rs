use std::collections::{HashMap, HashSet};

use aoc::grid::{lookup_offset, Posn};
use itertools::Itertools;

type Height = u8;
type File = Vec<Vec<Height>>;
fn parse(s: &str) -> File {
    s.lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_string().parse().unwrap_or(10))
                .collect_vec()
        })
        .collect_vec()
}

const INFILE: &str = "src/2024/10/in.txt";

fn main() {
    let f = aoc::must_read(INFILE);
    println!("part 1: {}", part_1(&parse(&f)));
    println!("part 2: {}", part_2(&parse(&f)));
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn part_1_ex() {
        use super::part_1;
        assert_eq!(
            part_1(&parse(
                "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"
            )),
            3
        );
        assert_eq!(
            part_1(&parse(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            )),
            36
        );
    }

    #[test]
    fn part_2_ex() {
        use super::part_2;
        assert_eq!(
            part_2(&parse(
                ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."
            )),
            3
        );
        assert_eq!(
            part_2(&parse(
                "..90..9
...1.98
...2..7
6543456
765.987
876....
987...."
            )),
            13
        );
        assert_eq!(
            part_2(&parse(
                "012345
123456
234567
345678
4.6789
56789."
            )),
            227
        );
        assert_eq!(
            part_2(&parse(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            )),
            81
        );
    }
}

fn part_1(data: &File) -> u64 {
    let mut reachable_nines: HashMap<Posn, HashSet<Posn>> = HashMap::new();
    // println!("data = {data:?}");
    for pass in (0..=9).rev() {
        for (i, row) in data.iter().enumerate() {
            for (j, height) in row.iter().enumerate() {
                if *height == pass {
                    println!("{pass} {i} {j}");
                    reachable_nines.insert(
                        (i, j),
                        if pass == 9 {
                            HashSet::from([(i, j)])
                        } else {
                            aoc::grid::DIRS_4
                                .iter()
                                .filter_map(|step| lookup_offset(&(i, j), &step, data))
                                .filter(|(_, nv)| **nv == height + 1)
                                .filter_map(|(npos, _)| reachable_nines.get(&npos))
                                .flat_map(|x| x)
                                .cloned()
                                .collect::<HashSet<_>>()
                        },
                    );
                }
            }
        }
    }
    println!("{reachable_nines:?}");
    reachable_nines
        .iter()
        .filter(|(posn, _)| data[posn.0][posn.1] == 0)
        .map(|(_, nines)| nines.len() as u64)
        .sum()
}

fn part_2(data: &File) -> u64 {
    let mut ratings: HashMap<Posn, u8> = HashMap::new();
    // println!("data = {data:?}");
    for pass in (0..=9).rev() {
        for (i, row) in data.iter().enumerate() {
            for (j, height) in row.iter().enumerate() {
                if *height == pass {
                    println!("{pass} {i} {j}");
                    ratings.insert(
                        (i, j),
                        if pass == 9 {
                            1
                        } else {
                            aoc::grid::DIRS_4
                                .iter()
                                .filter_map(|step| lookup_offset(&(i, j), step, data))
                                .filter(|(_, nv)| **nv == height + 1)
                                .filter_map(|(npos, _)| ratings.get(&npos))
                                .sum()
                        },
                    );
                }
            }
        }
    }
    println!("{ratings:?}");
    ratings
        .iter()
        .filter(|(posn, _)| data[posn.0][posn.1] == 0)
        .map(|(_, rating)| *rating as u64)
        .sum()
}
