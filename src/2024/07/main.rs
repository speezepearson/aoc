use itertools::{repeat_n, Itertools as _};
use nom::InputIter as _;

fn main() {
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
