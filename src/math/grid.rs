//each plane is sliced into a 15x15 grid
//index 0 is mapped to (-7, 7) while index 224 is (7, 7)

use std::collections::HashMap;

const PLANE_LENGTH: i32 = 15;

pub struct MyGrid {
    //index = z, value = vector of x coordinates
    pub coordinate_system: HashMap<Vec<i32>, bool> ,
}

impl Default for MyGrid {
    fn default() -> Self {
        let mut grid_occupied: HashMap<Vec<i32>, bool> = HashMap::new();
        for i in -7..8{
            for j in -7..8{
                grid_occupied.insert(vec![i, j], false);
            }
        }
        MyGrid {
            coordinate_system: grid_occupied
        }
    }
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Index {
    pub index: i32,
}

pub fn index_to_coord(index: Index) -> Coordinate {
    let modulus: i32 = index.index % PLANE_LENGTH;
    let x: f64 = (((PLANE_LENGTH - 1) * -1 / 2) + modulus).into();

    let divide: i32 = index.index / PLANE_LENGTH;
    let z: f64 = (((PLANE_LENGTH - 1) * -1 / 2) + divide).into();

    Coordinate { x, y: 0.0, z }
}

fn convert(x: f64) -> i32 {
    x as i32
}

pub fn coord_to_index(coord: Coordinate) -> Index {
    let mut mods: HashMap<i32, Vec<i32>> = HashMap::new();
    //still have to complete
    mods.insert(
        0,
        vec![
            0, 15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180, 195, 210,
        ],
    );

    //still have to complete
    let mut quotients: HashMap<i32, Vec<i32>> = HashMap::new();
    quotients.insert(0, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);

    let x_coord = convert(coord.x);
    let z_coord = convert(coord.y);
    let modulus: i32 = (((2 * x_coord) + PLANE_LENGTH - 1) / 2).abs();
    let divide: i32 = (((2 * z_coord) + PLANE_LENGTH - 1) / 2).abs();

    let vec1 = mods.get(&modulus).unwrap();
    let vec2 = quotients.get(&divide).unwrap();

    let mut index: i32 = 0;
    for i in vec1.iter() {
        if vec2.contains(i) {
            index = *i;
            break;
        }
    }

    Index { index }
}
