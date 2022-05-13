pub enum Cell {
    Covered(Box<Cell>),
    Bomb,
    Number(i8)
}

pub fn view(v : Cell) -> String {
    match v {
        Cell::Bomb => String::from("*"),
        Cell::Number(x) => format!("{}", x),
        Cell::Covered(_) => String::from("."),
    }
}
