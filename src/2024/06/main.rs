use std::{collections::HashSet, time::SystemTime};

use aoc::grid::{add_posn, DPosn, Posn};

fn main() {
    let datastr = std::fs::read_to_string("src/2024/06/in.txt").unwrap();

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
        dir: DPosn,
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

    fn rotate_90deg_right(dir: &DPosn) -> DPosn {
        (dir.1, -dir.0)
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
