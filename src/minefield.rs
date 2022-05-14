use std::collections::{hash_map, HashMap};

use super::mineitem::{*};
struct Minefield {
    cells:hash_map::HashMap<(i8, i8), Cell>,
    width: i8,
    height: i8
}

fn build_minefield (width: i8, height:i8 ) -> Minefield {
    let mut ret = Minefield { 
        width,
        height,
        cells : HashMap::new()
    };

    for a in 0..height {
        for b in 0..width {
            ret.cells.insert((a, b), Cell::Covered(Box::new(Cell::Number(0))));
        };
    };

    return ret;
                    
}