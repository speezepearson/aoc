use itertools::Itertools as _;

type File = Vec<u8>;

const INFILE: &str = "src/2024/09/in.txt";
const TESTFILE: &str = "src/2024/09/test.txt";

fn main() {
    let f = aoc::must_read(INFILE);
    println!("part 1: {}", part_1(parse(&f)));
    println!("part 2: {}", part_2(parse(&f)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frag_compact() {
        assert_eq!(
            fmt_tape(&frag_compact(&unpack(parse(&aoc::must_read(TESTFILE))))),
            "0099811188827773336446555566.............."
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(
            checksum(
                &"0099811188827773336446555566.............."
                    .chars()
                    .map(|c| c.to_string().parse::<u64>().ok())
                    .collect_vec()
            ),
            1928
        );
    }
}

fn parse(s: &str) -> File {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

type Id = u64;
type Tape = Vec<Option<Id>>;
fn fmt_tape(tape: &Tape) -> String {
    tape.iter()
        .map(|id| id.map(|x| x.to_string()).unwrap_or(".".to_string()))
        .collect_vec()
        .join("")
}
fn frag_compact(tape: &Tape) -> Tape {
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
fn checksum(tape: &Tape) -> u64 {
    tape.iter()
        .enumerate()
        .map(|(i, id)| match id {
            None => 0,
            Some(id) => i as u64 * id,
        })
        .sum()
}

fn part_1(data: File) -> u64 {
    let tape = unpack(data);
    let compacted = frag_compact(&tape);
    eprintln!("compacted = {}", fmt_tape(&compacted));
    checksum(&compacted)
}

fn part_2(data: File) -> u64 {
    let tape = unpack(data);

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
                    eprintln!("{}", fmt_tape(&working_copy));
                }
            }
        }
        eprintln!("processed {chunks_processed}/{n_chunks}");
        chunks_processed += 1;
    }
    eprintln!("compacted = {}", fmt_tape(&working_copy));

    checksum(&working_copy)
}

fn unpack(data: File) -> Tape {
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
}
