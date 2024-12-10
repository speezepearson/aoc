use nom::IResult;

fn main() {
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
