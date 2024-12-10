use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools as _};
use num::integer::gcd;

fn main() {
    let datastr = std::fs::read_to_string("p8.in.txt").unwrap();
    // let datastr = std::fs::read_to_string("p8.test.txt").unwrap();
    type Posn = (usize, usize);
    let data: Vec<Vec<char>> = datastr
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    let dims = (data.len(), data[0].len());

    let antenna_locs_by_freq: HashMap<char, HashSet<Posn>> = data
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(j, &c)| if c == '.' { None } else { Some((i, j, c)) })
        })
        .fold(HashMap::new(), |mut m, (i, j, c)| {
            m.entry(c).or_insert_with(HashSet::new).insert((i, j));
            m
        });
    println!("antenna locs {antenna_locs_by_freq:?}");
    println!(
        "antenna counts {:?}",
        antenna_locs_by_freq
            .iter()
            .map(|(&k, vs)| (k, vs.len()))
            .collect_vec()
    );

    {
        println!("-------------- part 1 -------------");
        let mut antinode_locs: HashSet<Posn> = HashSet::new();
        for (_, locs) in &antenna_locs_by_freq {
            for (a, b) in iproduct!(locs, locs) {
                if a == b {
                    continue;
                } else {
                    match (
                        b.0.checked_add_signed(b.0 as isize - a.0 as isize),
                        b.1.checked_add_signed(b.1 as isize - a.1 as isize),
                    ) {
                        (Some(an0), Some(an1)) if an0 < dims.0 && an1 < dims.1 => {
                            antinode_locs.insert((an0, an1));
                        }
                        _ => {}
                    }
                }
            }
        }
        println!(
            "{}",
            data.iter()
                .enumerate()
                .map(|(i, l)| l
                    .iter()
                    .enumerate()
                    .map(|(j, &c)| if antinode_locs.contains(&(i, j)) {
                        "!".to_string()
                    } else {
                        c.to_string()
                    })
                    .collect_vec()
                    .join(""))
                .collect_vec()
                .join("\n")
        );
        let answer = antinode_locs.len();
        println!("{answer:?}");
        println!("-----------------------------------");
    }

    {
        println!("-------------- part 2 -------------");
        let mut antinode_locs: HashSet<Posn> = HashSet::new();
        for (_, locs) in &antenna_locs_by_freq {
            if locs.len() > 1 {
                for loc in locs {
                    antinode_locs.insert(*loc);
                }
            }
            for (a, b) in iproduct!(locs, locs) {
                if a == b {
                    continue;
                }
                let d = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
                let d = (d.0 / gcd(d.0, d.1), d.1 / gcd(d.0, d.1));
                for i in 0.. {
                    let mut done = true;
                    for sign in [-1, 1].iter() {
                        match (
                            a.0.checked_add_signed(sign * i * d.0),
                            a.1.checked_add_signed(sign * i * d.1),
                        ) {
                            (Some(an0), Some(an1)) if an0 < dims.0 && an1 < dims.1 => {
                                antinode_locs.insert((an0, an1));
                                done = false;
                            }
                            _ => {}
                        }
                    }
                    if done {
                        break;
                    }
                }
            }
        }
        println!(
            "{}",
            data.iter()
                .enumerate()
                .map(|(i, l)| l
                    .iter()
                    .enumerate()
                    .map(|(j, &c)| if antinode_locs.contains(&(i, j)) {
                        "!".to_string()
                    } else {
                        c.to_string()
                    })
                    .collect_vec()
                    .join(""))
                .collect_vec()
                .join("\n")
        );
        let answer = antinode_locs.len();
        println!("{answer:?}");
        println!("-----------------------------------");
    }
}
