use helloworld::minefield;
use helloworld::mineitem;


#[test]
fn should_create() {
    let sut = minefield::Minefield{
        cells : [Cell::Covered(Box::new(Cell::Number(0)))]
    };
}
