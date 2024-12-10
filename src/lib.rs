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

pub mod grid {
    pub type Posn = (usize, usize);
    pub type DPosn = (isize, isize);
    pub const DIRS_4: [DPosn; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    pub const DIAGS: [DPosn; 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
    pub const DIRS_8: [DPosn; 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    pub fn add_posn(old: &Posn, dir: &DPosn, limits: &(usize, usize)) -> Option<Posn> {
        match (
            old.0.checked_add_signed(dir.0),
            old.1.checked_add_signed(dir.1),
        ) {
            (Some(i), Some(j)) if i < limits.0 && j < limits.1 => Some((i, j)),
            _ => None,
        }
    }

    pub fn add_posn_in<Item, Row, Grid>(old: &Posn, dir: &DPosn, grid: &Grid) -> Option<Posn>
    where
        Grid: AsRef<[Row]>,
        Row: AsRef<[Item]>,
    {
        add_posn(
            old,
            dir,
            &(grid.as_ref().len(), grid.as_ref()[0].as_ref().len()),
        )
    }

    pub fn lookup_offset<'a, Item, Row, Grid>(
        posn: &Posn,
        delta: &DPosn,
        grid: &'a Grid,
    ) -> Option<(Posn, &'a Item)>
    where
        Grid: AsRef<[Row]> + 'a,
        Row: AsRef<[Item]> + 'a,
    {
        let ni = posn.0.checked_add_signed(delta.0)?;
        let row = grid.as_ref().get(ni)?;
        let nj = posn.1.checked_add_signed(delta.1)?;
        let item = row.as_ref().get(nj)?;
        Some(((ni, nj), item))
    }
}
