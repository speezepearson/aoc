use aoc::grid::{add_posn_in, lookup_offset, DPosn, Posn, DIAGS, DIRS_8};
use itertools::Itertools;

fn main() {
    let datastr = std::fs::read_to_string("src/2024/04/in.txt").unwrap();

    fn read(grid: &Vec<Vec<char>>, start: &Posn, dir: &DPosn, len: usize) -> Vec<char> {
        let mut res = vec![];
        let mut start = *start;
        let mut len = len;
        while len > 0 {
            len -= 1;
            if let Some(c) = grid.get(start.0).and_then(|l| l.get(start.1)) {
                res.push(*c);
                start = match add_posn_in(&start, &dir, &grid) {
                    Some(pos) => pos,
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

    {
        println!("-------------- part 1 -------------");
        let target: Vec<char> = "XMAS".chars().collect();
        let grid: Vec<Vec<char>> = datastr.lines().map(|l| l.chars().collect()).collect();
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                for delta in &DIRS_8 {
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
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if !(grid[i][j] == 'A') {
                    continue;
                }

                let neighbors: Vec<(Posn, &char)> =
                    match Option::from_iter(DIAGS.iter().map(|d| lookup_offset(&(i, j), d, &grid)))
                    {
                        Some(ns) => ns,
                        None => continue,
                    };
                let neighbors = neighbors
                    .iter()
                    .map(|(_, c)| c.to_string())
                    .collect_vec()
                    .join("");

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
