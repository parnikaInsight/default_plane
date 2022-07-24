use rand::prelude::*;

#[derive(Debug)]
pub struct Random{
    pub number: f64,
}

pub fn get_block_dimensions() -> f64{
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen(); // generates a float between 0 and 1
    x
}

pub fn get_block_location() -> f64{
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen(); // generates a float between 0 and 1
    x
}
   