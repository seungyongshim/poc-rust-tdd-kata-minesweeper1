pub enum Cell {
    Covered(Box<Cell>),
    Bomb,
    Number(i8)
}

impl Cell {
    pub fn to_string(&self) -> String {
        match self {
            Cell::Covered(_) => String::from("."),
            Cell::Bomb => String::from("*"),
            Cell::Number(x) => format!("{}", x),
        }
    }
    pub fn click(self) -> Cell {
        match self {
            Cell::Covered(x) => *x,
            x => x
        }
    }
}
