use std::collections::{HashMap, HashSet};

fn p1() {
    let datastr = std::fs::read_to_string("p1.in.txt").unwrap();
    let data: Vec<(i32, i32)> = datastr
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap().parse::<i32>().unwrap();
            let right = parts.next().unwrap().parse::<i32>().unwrap();
            if let None = parts.next() {
                (left, right)
            } else {
                panic!("too many parts")
            }
        })
        .collect();

    {
        println!("-------------- part 1 -------------");
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = data.iter().cloned().unzip();
        println!("left {left:?}");
        println!("right {right:?}");

        left.sort();
        println!("sorted left {left:?}");
        right.sort();
        println!("sorted right {right:?}");

        let pairs = left.iter().zip(right.iter()).collect::<Vec<_>>();
        println!("pairs {pairs:?}");
        let dists = pairs
            .iter()
            .map(|(&l, &r)| (l as i32 - r as i32).abs())
            .collect::<Vec<_>>();
        println!("dists {dists:?}");

        let total = dists.iter().sum::<i32>();
        println!("total {total:?}");
    }

    {
        println!("-------------- part 2 -------------");
        let (left, right): (Vec<i32>, Vec<i32>) = data.iter().cloned().unzip();
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

        let total: i32 = similarities.iter().cloned().sum();
        println!("total {total:?}");
    }
}

fn p2() {
    type Report = Vec<i32>;

    let datastr = std::fs::read_to_string("p2.in.txt").unwrap();
    let data: Vec<Report> = datastr
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    fn report_unsafety_reason(r: &Report) -> Option<(&str, usize)> {
        if r.len() < 2 {
            return None;
        }

        let sign = (r[1] - r[0]).signum();
        if sign == 0 {
            return Some(("contains delta 0", 1));
        }
        for i in 1..r.len() {
            let delta = r[i] - r[i - 1];
            if delta.signum() != sign {
                return Some(("nonmonotonic", i));
            }
            if delta.abs() > 3 {
                return Some(("delta > 3", i));
            }
        }

        return None;
    }

    {
        println!("-------------- part 1 -------------");
        let unsafety_reasons = data.iter().map(report_unsafety_reason).collect::<Vec<_>>();
        for (i, safety) in unsafety_reasons.iter().enumerate() {
            println!(
                "{} ({:?}...) -> {:?}",
                i,
                data[i].iter().take(6).collect::<Vec<_>>(),
                safety
            );
        }
        let answer = unsafety_reasons
            .iter()
            .filter(|reason| reason.is_none())
            .count();
        println!("answer {answer:?}");
    }

    {
        println!("-------------- part 2 -------------");
        fn is_safe_with_dampener(r: &Report) -> bool {
            (0..r.len())
                .map(|i| {
                    r.iter()
                        .take(i)
                        .chain(r.iter().skip(i + 1))
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .any(|r| report_unsafety_reason(&r).is_none())
        }
        let answer = data.iter().filter(|&r| is_safe_with_dampener(r)).count();
        println!("answer {answer:?}");
    }
}

fn p3() {
    let re =
        regex::Regex::new(r#"mul\(([0-9]{1,3}),([0-9]{1,3})\)"#).expect("regex compilation failed");
    let datastr = std::fs::read_to_string("p3.in.txt").unwrap();

    if false {
        println!("-------------- part 1 -------------");
        let mut total = 0;
        for (s, [x, y]) in re.captures_iter(&datastr).map(|m| m.extract()) {
            println!("found {}", s);
            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();
            total += x * y;
        }
        println!("{total:?}");
        println!("-----------------------------------");
    }

    {
        println!("-------------- part 2 -------------");
        let re = regex::Regex::new(r#"do\(\)()()|don't\(\)()()|mul\(([0-9]{1,3}),([0-9]{1,3})\)"#)
            .expect("regex compilation failed");
        let mut total = 0;
        let mut enabled = true;
        for (s, [x, y]) in re.captures_iter(&datastr).map(|m| m.extract()) {
            println!("found {}", s);
            if s == "do()" {
                enabled = true
            } else if s == "don't()" {
                enabled = false
            } else if enabled {
                let x: i32 = x.parse().unwrap();
                let y: i32 = y.parse().unwrap();
                total += x * y;
            }
        }
        println!("{total:?}");
        println!("-----------------------------------");
    }
}

fn main() {
    p3();
}
