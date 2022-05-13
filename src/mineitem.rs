
pub enum Cover {
    Covered(Cell),
    Uncovered(Cell)
} 
pub enum Cell {
    Bomb,
    Number(i8)
}

impl Cover {
    pub fn to_string(&self) -> String {
        match self {
            Cover::Covered(_) => String::from("."),
            Cover::Uncovered(x) => x.to_string(),
        }
    }
}

impl Cell {
    pub fn to_string(&self) -> String {
        match self {
            Cell::Bomb => String::from("*"),
            Cell::Number(x) => format!("{}", x),
        }
    }
}
