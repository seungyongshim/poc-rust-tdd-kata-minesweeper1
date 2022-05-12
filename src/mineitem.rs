pub enum Cell {
    Covered {inner: Box<Cell>},
    Bomb,
    Number {value: i8}
}

pub fn view(v : Cell) -> String {
    match v {
        Bomb => String::from("*"),
        Number {v} => v
    }
}

pub fn hello() -> i8 {
    1
}
