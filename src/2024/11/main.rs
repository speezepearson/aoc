use std::collections::HashMap;

use itertools::Itertools;

type File = Stones;
fn parse(s: &str) -> File {
    s.split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect_vec()
}

type Stone = u64;
type Stones = Vec<Stone>;
fn step(stones: &Stones) -> Stones {
    let mut res = vec![];
    for &stone in stones {
        if stone == 0 {
            res.push(1);
        } else if let Some((a, b)) = break_even_digits(stone) {
            res.push(a);
            res.push(b);
        } else {
            res.push(stone * 2024);
        }
    }
    res
}
fn break_even_digits(n: Stone) -> Option<(Stone, Stone)> {
    let n_digits = if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    };
    if n_digits % 2 != 0 {
        return None;
    }

    // n = 0    ->   ndig = 1   ->   mod = --
    // n = 10   ->   ndig = 2   ->   mod = 10
    // n = 35   ->   ndig = 2   ->   mod = 10
    // n = 1234 ->   ndig = 4   ->   mod = 100
    let modulus = 10_u64.pow(n_digits / 2);
    Some((n / modulus, n % modulus))
}

const INFILE: &str = "src/2024/11/in.txt";

fn main() {
    let f = aoc::must_read(INFILE);
    println!("part 1: {}", part_1(&parse(&f)));
    println!("part 2: {}", part_2(&parse(&f)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_break_even_digits() {
        assert_eq!(break_even_digits(4), None);
        assert_eq!(break_even_digits(12), Some((1, 2)));
        assert_eq!(break_even_digits(1234), Some((12, 34)));
    }

    #[test]
    fn test_step() {
        assert_eq!(
            step(&vec![0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );
        let v = vec![125, 17];
        let v = step(&v);
        assert_eq!(v, vec![253000, 1, 7]);
        let v = step(&v);
        assert_eq!(v, vec![253, 0, 2024, 14168]);
        let v = step(&v);
        assert_eq!(v, vec![512072, 1, 20, 24, 28676032]);
        let v = step(&v);
        assert_eq!(v, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        let v = step(&v);
        assert_eq!(
            v,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        let v = step(&v);
        assert_eq!(
            v,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn part_1_ex() {
        assert_eq!(part_1(&parse("125 17")), 55312);
    }

    #[test]
    fn part_2_ex() {
        // no test cases available :(
    }
}

fn part_1(data: &File) -> u64 {
    let mut stones = data.clone();
    for i in 0..25 {
        println!("{i}");
        stones = step(&stones);
    }
    stones.len() as u64
}

fn part_2(data: &File) -> u64 {
    type NStones = u64;
    type NSteps = u16;
    type ExpansionCache = HashMap<(Stone, NSteps), NStones>;
    fn compute(cache: &mut ExpansionCache, stone: Stone, n_steps: NSteps) -> NStones {
        if n_steps == 0 {
            1
        } else if let Some(res) = cache.get(&(stone, n_steps)) {
            *res
        } else {
            if n_steps % 35 == 0 {
                println!("{stone} {n_steps}");
            }
            let next = step(&vec![stone]);
            let res = next.iter().map(|&s| compute(cache, s, n_steps - 1)).sum();
            cache.insert((stone, n_steps), res);
            res
        }
    }

    let mut cache: ExpansionCache = HashMap::new();
    data.iter().map(|&s| compute(&mut cache, s, 75)).sum()
}
