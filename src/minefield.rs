use std::{collections::{hash_map, HashMap}, ops::Generator, iter::FlatMap};

use super::mineitem::{*};
struct Minefield {
    cells:hash_map::HashMap<(i8, i8), Cell>,
    width: i8,
    height: i8
}

fn build_minefield (width: i8, height:i8 ) -> Minefield {
    let ret = Minefield { 
        width,
        height,
        cells : HashMap::new()
    };

    [0..width].m
}