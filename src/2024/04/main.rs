fn main() {
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
