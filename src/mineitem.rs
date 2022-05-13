pub enum Cell {
    Covered(Box<Cell>),
    Bomb,
    Number(u8)
}

impl Cell {
    pub fn to_char(&self) -> char {
        match self {
            Cell::Covered(_) => '.',
            Cell::Bomb => '*',
            Cell::Number(x) => char::from_digit(*x as u32, 10).unwrap()
        }
    }
    pub fn click(self) -> Cell {
        match self {
            Cell::Covered(x) => *x,
            x => x
        }
    }
}
