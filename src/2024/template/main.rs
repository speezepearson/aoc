type File = Vec<u8>;
fn parse(s: &str) -> File {
    todo!();
}

const INFILE: &str = "__TEMPLATE_HERE__/in.txt";
const TESTFILE: &str = "__TEMPLATE_HERE__/test.txt";

fn main() {
    let f = aoc::must_read(INFILE);
    println!("part 1: {}", part_1(parse(&f)));
    println!("part 2: {}", part_2(parse(&f)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn foo() {
        assert_eq!(todo!(), todo!());
    }
}

fn part_1(data: File) -> u64 {
    todo!();
}

fn part_2(data: File) -> u64 {
    todo!();
}
