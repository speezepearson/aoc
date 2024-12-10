fn main() {
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
