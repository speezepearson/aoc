pub fn run() {
    fn parse_match(s: &str) -> i64 {
        match s {
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => s.parse().unwrap(),
        }
    }

    let datastr = std::fs::read_to_string("src/y2023/p1.in.txt").expect("Unable to read file");

    let mut total = 0;
    for line in datastr.lines() {
        println!("{line}");
        let first = (0..line.len() + 1)
            .filter_map(|i| {
                let rest = &line[i..];
                if rest.starts_with("0") || rest.starts_with("zero") {
                    Some(0)
                } else if rest.starts_with("1") || rest.starts_with("one") {
                    Some(1)
                } else if rest.starts_with("2") || rest.starts_with("two") {
                    Some(2)
                } else if rest.starts_with("3") || rest.starts_with("three") {
                    Some(3)
                } else if rest.starts_with("4") || rest.starts_with("four") {
                    Some(4)
                } else if rest.starts_with("5") || rest.starts_with("five") {
                    Some(5)
                } else if rest.starts_with("6") || rest.starts_with("six") {
                    Some(6)
                } else if rest.starts_with("7") || rest.starts_with("seven") {
                    Some(7)
                } else if rest.starts_with("8") || rest.starts_with("eight") {
                    Some(8)
                } else if rest.starts_with("9") || rest.starts_with("nine") {
                    Some(9)
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        let last = (0..line.len() + 1)
            .filter_map(|i| {
                let rest = &line[..i];
                if rest.ends_with("0") || rest.ends_with("zero") {
                    Some(0)
                } else if rest.ends_with("1") || rest.ends_with("one") {
                    Some(1)
                } else if rest.ends_with("2") || rest.ends_with("two") {
                    Some(2)
                } else if rest.ends_with("3") || rest.ends_with("three") {
                    Some(3)
                } else if rest.ends_with("4") || rest.ends_with("four") {
                    Some(4)
                } else if rest.ends_with("5") || rest.ends_with("five") {
                    Some(5)
                } else if rest.ends_with("6") || rest.ends_with("six") {
                    Some(6)
                } else if rest.ends_with("7") || rest.ends_with("seven") {
                    Some(7)
                } else if rest.ends_with("8") || rest.ends_with("eight") {
                    Some(8)
                } else if rest.ends_with("9") || rest.ends_with("nine") {
                    Some(9)
                } else {
                    None
                }
            })
            .last()
            .unwrap();
        let n = first * 10 + last;
        println!("found {first}...{last} => {n}");
        total += n;
    }

    println!("{total}");
}
