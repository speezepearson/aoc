use itertools::{iproduct, repeat_n, Itertools};
use nom::{bytes::complete::tag, IResult, InputIter};
use num::integer::gcd;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    time::SystemTime,
};

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

pub fn p6() {
    let datastr = std::fs::read_to_string("p6.in.txt").unwrap();

    type Posn = (usize, usize);
    type Dir = (isize, isize);
    #[derive(Debug, PartialEq, Eq)]
    struct File {
        guard_start: Posn,
        blocks: HashSet<Posn>,
        dims: (usize, usize),
    }
    let file = File {
        guard_start: datastr
            .lines()
            .enumerate()
            .find_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .position(|(_, c)| c == '^')
                    .map(|j| (i, j))
            })
            .unwrap(),
        blocks: datastr
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(j, c)| if c == '#' { Some((i, j)) } else { None })
            })
            .collect(),
        dims: (
            datastr.lines().count(),
            datastr.lines().next().unwrap().len(),
        ),
    };
    println!("file = {file:?}");
    #[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
    struct State {
        posn: Posn,
        dir: Dir,
    }

    fn step(file: &File, state: &State) -> Option<State> {
        let dest = add_posn(&state.posn, &state.dir, &file.dims)?;
        if file.blocks.contains(&dest) {
            Some(State {
                posn: state.posn,
                dir: rotate_90deg_right(&state.dir),
            })
        } else {
            Some(State {
                posn: dest,
                dir: state.dir,
            })
        }
    }

    fn rotate_90deg_right(dir: &Dir) -> Dir {
        (dir.1, -dir.0)
    }
    fn add_posn(old: &Posn, dir: &Dir, dims: &(usize, usize)) -> Option<Posn> {
        match (
            old.0.checked_add_signed(dir.0),
            old.1.checked_add_signed(dir.1),
        ) {
            (Some(i), Some(j)) if i < dims.0 && j < dims.1 => Some((i, j)),
            _ => None,
        }
    }

    let start_state = State {
        posn: file.guard_start,
        dir: (-1, 0),
    };

    {
        println!("-------------- part 1 -------------");
        let mut state = start_state;
        let mut history = vec![state];
        while let Some(next) = step(&file, &state) {
            state = next;
            history.push(state);
            println!("stepped to {state:?}");
        }
        let answer = history.iter().map(|s| s.posn).collect::<HashSet<_>>().len();
        println!("{answer:?}");
        println!("-----------------------------------");
    }

    {
        println!("-------------- part 2 -------------");
        let mut state = State {
            posn: file.guard_start,
            dir: (-1, 0),
        };
        let mut base_history = vec![state];
        let mut loop_block_posns = HashSet::new();

        let mut last_print: Option<SystemTime> = None;
        while let Some(next) = step(&file, &state) {
            if next.posn != state.posn
                && !base_history.iter().any(|s| s.posn == next.posn)
                && does_loop(&file, &next.posn, &state)
            {
                loop_block_posns.insert(next.posn);
            }
            state = next;
            base_history.push(state);
            let now = SystemTime::now();
            match last_print {
                Some(t) if now.duration_since(t).unwrap().as_millis() < 1000 => {}
                _ => {
                    let visited = base_history.iter().map(|s| s.posn).collect::<HashSet<_>>();
                    println!(
                        "{}",
                        (0..file.dims.0)
                            .map(|i| (0..file.dims.1)
                                .map(|j| if (i, j) == file.guard_start {
                                    "^"
                                } else if loop_block_posns.contains(&(i, j)) {
                                    "O"
                                } else if file.blocks.contains(&(i, j)) {
                                    "#"
                                } else if visited.contains(&(i, j)) {
                                    "."
                                } else {
                                    " "
                                })
                                .collect::<Vec<_>>()
                                .join(""))
                            .collect::<Vec<_>>()
                            .join("\n")
                            + "\n---------------------------------------"
                    );
                    last_print = Some(now);
                }
            }
        }

        fn does_loop(file: &File, extra: &Posn, start: &State) -> bool {
            let file = File {
                blocks: {
                    let mut b = file.blocks.clone();
                    b.insert(*extra);
                    b
                },
                ..*file
            };
            let mut state = *start;
            let mut history = HashSet::from([state]);
            while let Some(next) = step(&file, &state) {
                if history.contains(&next) {
                    // println!("found loop!");
                    // let visited = history.iter().map(|s| s.posn).collect::<HashSet<_>>();
                    // println!(
                    //     "{}",
                    //     (0..file.dims.0)
                    //         .map(|i| (0..file.dims.1)
                    //             .map(|j| if (i, j) == start.posn {
                    //                 match start.dir {
                    //                     (-1, 0) => "^",
                    //                     (1, 0) => "v",
                    //                     (0, 1) => ">",
                    //                     (0, -1) => "<",
                    //                     _ => panic!("wat dir"),
                    //                 }
                    //             } else if &(i, j) == extra {
                    //                 "O"
                    //             } else if file.blocks.contains(&(i, j)) {
                    //                 "#"
                    //             } else if visited.contains(&(i, j)) {
                    //                 "."
                    //             } else {
                    //                 " "
                    //             })
                    //             .collect::<Vec<_>>()
                    //             .join(""))
                    //         .collect::<Vec<_>>()
                    //         .join("\n")
                    //         + "\n---------------------------------------"
                    // );
                    return true;
                }
                state = next;
                history.insert(state);
            }
            false
        }

        let visited = base_history.iter().map(|s| s.posn).collect::<HashSet<_>>();
        println!(
            "{}",
            (0..file.dims.0)
                .map(|i| (0..file.dims.1)
                    .map(|j| if (i, j) == file.guard_start {
                        "^"
                    } else if loop_block_posns.contains(&(i, j)) {
                        "O"
                    } else if file.blocks.contains(&(i, j)) {
                        "#"
                    } else if visited.contains(&(i, j)) {
                        "."
                    } else {
                        " "
                    })
                    .collect::<Vec<_>>()
                    .join(""))
                .collect::<Vec<_>>()
                .join("\n")
        );

        assert!(loop_block_posns.iter().all(|extra| does_loop(
            &file,
            extra,
            base_history
                .iter()
                .find(|s| step(&file, s).unwrap().posn == *extra)
                .unwrap()
        )));

        let answer = loop_block_posns.len();
        println!("{answer:?}");
        println!("-----------------------------------");
    }
}

pub fn p7() {
    let datastr = std::fs::read_to_string("p7.in.txt").unwrap();
    let data: Vec<(i64, Vec<i64>)> = datastr
        .lines()
        .map(|l| {
            let (ans, args) = l.split_at(l.position(|c| c == ':').unwrap());
            (
                ans.parse().unwrap(),
                args.trim()
                    .strip_prefix(":")
                    .unwrap()
                    .split_whitespace()
                    .map(|w| w.parse().unwrap())
                    .collect(),
            )
        })
        .collect();
    // let data = vec![(9499727921208, vec![2, 5, 48, 82, 4, 678, 564, 49])];

    #[derive(Debug, PartialEq, Eq)]
    enum Op {
        Add,
        Mul,
        Concat,
    }
    impl std::fmt::Display for Op {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Add => f.write_str("+"),
                Self::Mul => f.write_str("*"),
                Self::Concat => f.write_str("|"),
            }
        }
    }
    impl Op {
        fn call(&self, lhs: i64, rhs: i64) -> i64 {
            match self {
                Self::Add => lhs + rhs,
                Self::Mul => lhs * rhs,
                Self::Concat => vec![lhs.to_string(), rhs.to_string()]
                    .join("")
                    .parse()
                    .unwrap(),
            }
        }
    }

    if false {
        println!("-------------- part 1 -------------");

        let mut answer = 0;
        for (target, args) in &data {
            println!("{}: {:?}", target, args);
            for ops in repeat_n([Op::Add, Op::Mul].iter(), args.len() - 1).multi_cartesian_product()
            {
                let actual = args
                    .iter()
                    .skip(1)
                    .zip(&ops)
                    .fold(args[0], |acc, (&x, op)| op.call(acc, x));
                if actual == *target {
                    println!(
                        "hit: {} = {} {}",
                        target,
                        args[0],
                        args.iter()
                            .skip(1)
                            .zip(&ops)
                            .map(|(n, op)| format!("{op} {n}"))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                    answer += target;
                    break;
                }
            }
        }

        println!("{answer:?}");
        println!("-----------------------------------");
    }

    {
        println!("-------------- part 2 -------------");

        let mut answer = 0;
        for (target, args) in &data {
            println!("{target} {args:?}");
            for ops in repeat_n([Op::Add, Op::Mul, Op::Concat].iter(), args.len() - 1)
                .multi_cartesian_product()
            {
                // println!("");
                let actual = args
                    .iter()
                    .skip(1)
                    .zip(&ops)
                    .fold(args[0], |acc, (&x, op)| {
                        // println!("  {acc} {op} {x} = {}", op.call(acc, x));
                        op.call(acc, x)
                    });
                if actual == *target {
                    println!(
                        "hit: {} = {} {}",
                        target,
                        args[0],
                        args.iter()
                            .skip(1)
                            .zip(&ops)
                            .map(|(n, op)| format!("{op} {n}"))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                    answer += target;
                    break;
                }
            }
        }
        println!("{answer:?}");
        println!("-----------------------------------");
    }
}

pub fn p8() {
    let datastr = std::fs::read_to_string("p8.in.txt").unwrap();
    // let datastr = std::fs::read_to_string("p8.test.txt").unwrap();
    type Posn = (usize, usize);
    type DPosn = (isize, isize);
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

// struct BidiMap<'a, L, R> {
//     // left: HashSet<&'a L>,
//     // right: HashSet<&'a R>,
//     fwd: HashMap<&'a L, &'a R>,
//     bak: HashMap<&'a R, &'a L>,
// }
// impl<'a, L, R> BidiMap<'a, L, R>
// where
//     L: Hash + Eq,
//     R: Hash + Eq,
// {
//     fn new() -> Self {
//         Self {
//             // left: HashSet::new(),
//             // right: HashSet::new(),
//             fwd: HashMap::new(),
//             bak: HashMap::new(),
//         }
//     }
//     fn get_fwd(&self, l: &L) -> Option<&R> {
//         self.fwd.get(l).map(|&x| x)
//     }
//     fn get_bak(&self, r: &R) -> Option<&L> {
//         self.bak.get(r).map(|&x| x)
//     }
//     fn add(&mut self, l: &'a L, r: &'a R) -> bool {
//         if self.fwd.contains_key(l) || self.bak.contains_key(r) {
//             false
//         } else {
//             // self.left.insert(l);
//             // self.right.insert(r);
//             self.fwd.insert(l, r);
//             self.bak.insert(r, l);
//             true
//         }
//     }
// }
// pub fn foo() {
//     let mut m: BidiMap<String, String> = BidiMap::new();
//     let s1 = "foo".to_string();
//     let s2 = "bar".to_string();
//     let s11 = "foo".to_string();
//     m.add(&s1, &s2);
//     println!(
//         "{:?} {:?} {:?}",
//         m.get_fwd(&s1),
//         m.get_bak(&s2),
//         m.get_fwd(&s11)
//     );
// }

pub fn p9() {
    let datastr = std::fs::read_to_string("p9.in.txt").unwrap();
    let data: Vec<u8> = datastr
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    type Tape = Vec<Option<u64>>;

    let tape: Tape = {
        let mut tape = vec![];
        let mut next_id = 0;
        for (i, &size) in data.iter().enumerate() {
            if i % 2 == 0 {
                for _ in 0..size {
                    tape.push(Some(next_id));
                }
                next_id += 1;
            } else {
                for _ in 0..size {
                    tape.push(None);
                }
            }
        }
        tape
    };
    eprintln!("tape = {tape:?}");

    {
        println!("-------------- part 1 -------------");

        fn compact(tape: &Tape) -> Tape {
            let mut res = tape.clone();
            let mut left = 0;
            let mut right = tape.len() - 1;
            while left < right {
                // eprintln!("l={left}/{:?}, r={right}/{:?}", tape[left], tape[right]);
                match (res[left], res[right]) {
                    (None, Some(_)) => {
                        res.swap(left, right);
                    }
                    (None, None) => right -= 1,
                    (Some(_), _) => left += 1,
                }
            }
            res
        }

        let compacted = compact(&tape);
        eprintln!("compacted = {compacted:?}");

        let answer: u64 = compacted
            .iter()
            .enumerate()
            .map(|(i, id)| match id {
                None => 0,
                Some(id) => i as u64 * id,
            })
            .sum();
        println!("{answer:?}");
        println!("-----------------------------------");
    }

    {
        println!("-------------- part 2 -------------");

        let gaps: Vec<Vec<usize>> = {
            let mut gaps: Vec<Vec<usize>> = (0..10).map(|_| vec![]).collect();
            for chunk in tape
                .iter()
                .enumerate()
                .collect::<Vec<_>>()
                .chunk_by(|(_, a), (_, b)| a.is_none() == b.is_none())
            {
                if chunk[0].1.is_none() {
                    gaps[chunk.len()].push(chunk[0].0)
                }
            }
            gaps
        };
        eprintln!("gaps: {gaps:?}");

        let mut working_copy = tape.clone();
        let n_chunks = tape
            .iter()
            .enumerate()
            .chunk_by(|(_, &id)| id)
            .into_iter()
            .filter_map(|(id, mut g)| match id {
                Some(id) => Some((id, g.next().unwrap().0, g.count() + 1)),
                None => None,
            })
            .count();
        eprintln!(
            "   {}",
            working_copy
                .iter()
                .map(|id| id.map(|x| x.to_string()).unwrap_or(".".to_string()))
                .collect_vec()
                .join("")
        );
        let mut chunks_processed = 0;
        let mut chunks_moved = 0;
        for (_src_id, src_start, src_len) in tape
            .iter()
            .enumerate()
            .chunk_by(|(_, &id)| id)
            .into_iter()
            .filter_map(|(id, mut g)| match id {
                Some(id) => Some((id, g.next().unwrap().0, g.count() + 1)),
                None => None,
            })
            .collect::<Vec<_>>()
            .iter()
            .rev()
        {
            let first_big_gap_start = working_copy
                .iter()
                .enumerate()
                .chunk_by(|(_, &id)| id)
                .into_iter()
                .filter_map(|(id, mut group)| match id {
                    None => {
                        let start = group.next().unwrap().0;
                        let len = group.count() + 1;
                        if &start < src_start && &len >= src_len {
                            Some(start)
                        } else {
                            None
                        }
                    }
                    Some(_) => None,
                })
                .next();
            match first_big_gap_start {
                None => {}
                Some(gap_start) => {
                    for i in 0..*src_len {
                        working_copy.swap(gap_start + i, src_start + i);
                    }
                    chunks_moved += 1;
                    if chunks_moved % 1000 == 0 {
                        eprintln!(
                            "-> {}",
                            working_copy
                                .iter()
                                .map(|id| id.map(|x| x.to_string()).unwrap_or(".".to_string()))
                                .collect_vec()
                                .join("")
                        );
                    }
                }
            }
            eprintln!("processed {chunks_processed}/{n_chunks}");
            chunks_processed += 1;
        }
        eprintln!("compacted = {working_copy:?}");

        let answer: u64 = working_copy
            .iter()
            .enumerate()
            .map(|(i, id)| match id {
                None => 0,
                Some(id) => i as u64 * id,
            })
            .sum();
        println!("{answer:?}");
        println!("-----------------------------------");
    }
}
