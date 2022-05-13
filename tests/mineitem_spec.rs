use helloworld::mineitem::{*};

#[test]
fn should_bomb() {
    let sut = Cell::Bomb;
    let ret = sut.to_char();
    assert_eq!('*', ret);
}

#[test]
fn should_numver() {
    let sut = Cell::Number(1);
    let ret = sut.to_char();
    assert_eq!('1', ret);
}

#[test]
fn should_cover() {
    let sut = Cell::Covered(Box::new(Cell::Bomb));
    let ret = sut.to_char();
    assert_eq!('.', ret);
}

#[test]
fn should_click() {
    let sut = Cell::Covered(Box::new(Cell::Bomb));
    let ret = sut.click().to_char();
    assert_eq!('*', ret);
}