use rand::prelude::*;

#[derive(Debug)]
pub struct Random{
    pub x: f32,
}

pub fn get_block_dimensions() -> f32{
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen(); // generates a float between 0 and 1
    x
}

pub fn get_block_location() -> f32{
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen(); // generates a float between 0 and 1
    x
}
   