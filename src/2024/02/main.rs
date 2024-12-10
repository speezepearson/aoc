fn main() {
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
