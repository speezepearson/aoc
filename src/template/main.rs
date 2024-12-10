type File = ();
fn parse(s: &str) -> File {
    todo!();
}

const INFILE: &str = "__TEMPLATE_HERE__/in.txt";

fn main() {
    let f = aoc::must_read(INFILE);
    println!("part 1: {}", part_1(parse(&f)));
    println!("part 2: {}", part_2(parse(&f)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_ex() {
        assert_eq!(part_1(parse("...")), 9999999);
        assert_eq!(part_1(parse("...")), 9999999);
        assert_eq!(part_1(parse("...")), 9999999);
    }

    #[test]
    fn part_2_ex() {
        assert_eq!(part_2(parse("...")), 9999999);
        assert_eq!(part_2(parse("...")), 9999999);
        assert_eq!(part_2(parse("...")), 9999999);
    }
}

fn part_1(data: &File) -> u64 {
    todo!();
}

fn part_2(data: &File) -> u64 {
    todo!();
}
