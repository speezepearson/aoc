use nom::{bytes::complete::tag, IResult};
use std::collections::HashMap;

pub fn template() {
    let datastr = std::fs::read_to_string("pn.in.txt").unwrap();

    #[derive(Debug, PartialEq, Eq)]
    struct File {}
    fn parse_file(input: &str) -> IResult<&str, File> {
        nom::combinator::map(
            nom::sequence::terminated(tag("TODO"), nom::combinator::eof),
            |_todo| File {
                // TODO
            },
        )(input)
    }

    let (_, file) = match parse_file(&datastr) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("oh no: {}", e);
            return;
        }
    };
    println!("file = {file:?}");

    {
        println!("-------------- part 1 -------------");
        let answer = ();
        println!("{answer:?}");
        println!("-----------------------------------");
    }

    {
        println!("-------------- part 2 -------------");
        let answer = ();
        println!("{answer:?}");
        println!("-----------------------------------");
    }
}

pub fn p1() {
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

pub fn p2() {
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

pub fn p3() {
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

pub fn p4() {
    let datastr = std::fs::read_to_string("p4.in.txt").unwrap();

    fn read(
        grid: &Vec<Vec<char>>,
        start: &(usize, usize),
        dir: &(i32, i32),
        len: usize,
    ) -> Vec<char> {
        let mut res = vec![];
        let mut start = *start;
        let mut len = len;
        while len > 0 {
            len -= 1;
            if let Some(c) = grid.get(start.0).and_then(|l| l.get(start.1)) {
                res.push(*c);
                start = match (
                    (start.0 as i32 + dir.0).try_into(),
                    (start.1 as i32 + dir.1).try_into(),
                ) {
                    (Ok(x), Ok(y)) => (x, y),
                    _ => {
                        return res;
                    }
                }
            } else {
                return res;
            }
        }
        res
    }

    if false {
        println!("-------------- part 1 -------------");
        let target: Vec<char> = "XMAS".chars().collect();
        let grid: Vec<Vec<char>> = datastr.lines().map(|l| l.chars().collect()).collect();
        let deltas: Vec<(i32, i32)> = (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .collect::<Vec<_>>();
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                for delta in &deltas {
                    let str = read(&grid, &(i, j), delta, 4);
                    if str == target {
                        println!("found at ({i},{j}) going {delta:?}");
                        count += 1;
                    }
                }
            }
        }

        println!("{count}");
        println!("-----------------------------------");
    }

    {
        println!("-------------- part 2 -------------");
        let grid: Vec<Vec<char>> = datastr.lines().map(|l| l.chars().collect()).collect();
        let mut count = 0;
        for i in 1..grid.len() - 1 {
            for j in 1..grid[i].len() - 1 {
                if !(grid[i][j] == 'A') {
                    continue;
                }
                let neighbors = [
                    grid[i - 1][j - 1],
                    grid[i - 1][j + 1],
                    grid[i + 1][j + 1],
                    grid[i + 1][j - 1],
                ]
                .iter()
                .collect::<String>();

                if neighbors == "SSMM"
                    || neighbors == "SMMS"
                    || neighbors == "MMSS"
                    || neighbors == "MSSM"
                {
                    println!("found at ({i},{j})");
                    count += 1;
                }
            }
        }

        println!("{count}");
        println!("-----------------------------------");
    }
}

pub fn p5() {
    let datastr = std::fs::read_to_string("p5.in.txt").unwrap();
    type PageNum = usize;
    type Rule = (PageNum, PageNum);
    type Update = Vec<PageNum>;
    fn obeys_rule(u: &Update, r: &Rule) -> bool {
        if let Some(i) = u.iter().position(|&x| x == r.0) {
            if let Some(j) = u.iter().position(|&x| x == r.1) {
                if j <= i {
                    return false;
                }
            }
        }
        true
    }

    #[derive(Debug, PartialEq, Eq)]
    struct File {
        rules: Vec<Rule>,
        updates: Vec<Update>,
    }
    use nom::bytes::complete::tag;
    fn parse_pagenum(input: &str) -> IResult<&str, PageNum> {
        nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse())(input)
    }
    fn parse_rule(input: &str) -> IResult<&str, Rule> {
        nom::sequence::separated_pair(parse_pagenum, tag("|"), parse_pagenum)(input)
    }
    fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
        nom::multi::separated_list0(tag("\n"), parse_rule)(input)
    }
    fn parse_update(input: &str) -> IResult<&str, Update> {
        nom::multi::separated_list0(tag(","), parse_pagenum)(input)
    }
    fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
        nom::multi::separated_list0(tag("\n"), parse_update)(input)
    }
    fn parse_file(input: &str) -> IResult<&str, File> {
        nom::combinator::map(
            nom::sequence::terminated(
                nom::sequence::separated_pair(parse_rules, tag("\n\n"), parse_updates),
                nom::combinator::eof,
            ),
            |(rules, updates)| File { rules, updates },
        )(input)
    }
    let (_, file) = match parse_file(&datastr) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("oh no: {}", e);
            return;
        }
    };
    println!("file = {file:?}");

    {
        println!("-------------- part 1 -------------");

        let ok_updates = file
            .updates
            .iter()
            .filter(|u| file.rules.iter().all(|r| obeys_rule(u, r)));
        let middles = ok_updates.map(|u| u[u.len() / 2]);

        let answer = middles.sum::<usize>();
        println!("{answer:?}");
        println!("-----------------------------------");
    }

    {
        println!("-------------- part 2 -------------");
        fn fix(u: &mut Update, rules: &Vec<Rule>) {
            loop {
                let mut all_good = true;
                for (first, second) in rules {
                    if let Some(i) = u.iter().position(|x| x == first) {
                        if let Some(j) = u.iter().position(|x| x == second) {
                            if j <= i {
                                u.swap(i, j);
                                all_good = false;
                            }
                        }
                    }
                }
                if all_good {
                    for r in rules {
                        debug_assert!(
                            obeys_rule(&u, &r),
                            "not actually a fixed point: {u:?} violates {r:?}"
                        );
                    }
                    return;
                }
            }
        }
        let broken_updates = file
            .updates
            .clone()
            .into_iter()
            .filter(|u| file.rules.iter().any(|r| !obeys_rule(&u, &r)));
        let fixed = broken_updates.map(|mut u| {
            fix(&mut u, &file.rules);
            u
        });
        let middles = fixed.map(|u| u[u.len() / 2]);

        let answer = middles.sum::<usize>();
        println!("{answer:?}");
        println!("-----------------------------------");
    }
}
