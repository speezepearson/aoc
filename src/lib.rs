pub fn must_read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub mod parse {
    use nom::IResult;
    pub fn decimal<T>(input: &str) -> IResult<&str, T>
    where
        T: std::str::FromStr,
    {
        nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<T>())(input)
    }
}
