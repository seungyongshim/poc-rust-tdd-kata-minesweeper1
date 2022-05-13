use helloworld::mineitem::{*};

#[test]
fn should_bomb() {
    let sut = Cell::Bomb;
    let ret = sut.to_string();
    assert_eq!("*", ret);
}

#[test]
fn should_numver() {
    let sut = Cell::Number(1);
    let ret = sut.to_string();
    assert_eq!("1", ret);
}

#[test]
fn should_cover() {
    let sut = Cover::Covered(Cell::Bomb);
    let ret = sut.to_string();
    assert_eq!(".", ret);

}